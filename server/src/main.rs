
#![allow(dead_code)]

mod util;
mod game;
mod server_internal;
mod hints;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use actix_web::HttpResponse;
use actix_web::web;

use crate::game::MAX_WORD_COUNT;

#[derive(PartialEq, Eq)]
enum GamePhase {
    Typing, Sabotaging, Restarting
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum HintType {
    Green, Yellow, Red, Gray, None
}

struct GameState {
    word_to_guess: String,
    current_turn: i64,
    current_phase: GamePhase
}

// NOTE: do not rename these enums' values! Serialization depend on the names.
#[derive(serde::Deserialize)]
enum Language {
    French, English
}
#[derive(serde::Deserialize)]
enum Mode {
    Normal, Hard
}
#[derive(serde::Deserialize)]
enum Attack {
    Sabotage, InvisibleSabotage, Espionage
}
#[derive(serde::Deserialize)]
enum AttackMode {
    OncePerTurn,
    Multiple {
        available_attacks: Vec<Attack>
    }
}

#[derive(serde::Deserialize)]
struct GameOptions {
    lang: Language,
    mode: Mode,
    attacks: AttackMode,
}

/// Settings for a player (thins that the player has configured)
#[derive(serde::Serialize, Clone)]
struct PlayerInfo {
    name: String,
}

struct SocketConnection {
    session: actix_ws::Session,
    stream: actix_ws::AggregatedMessageStream,
}

/// Think we know about a player in a game
struct Player {
    player_info: Option<PlayerInfo>, // None if not sent by the player yet
    messages_to_send: Vec<String>,
    connection_alive: bool,
    last_ping_time: std::time::Instant,
    typed_word_this_turn: Option<String>,
    letter_sabotaged_this_turn: Option<u64>,
    past_words: Vec<String>,
    ready_to_restart: bool,
}

struct RoomState {
    game_state: GameState,
    host_player: Player,
    other_player: Option<Player>,
    join_code: String,
    game_started: bool,
    game_options: GameOptions,
}

struct AppState {
    rooms: Mutex<HashMap<String, Arc<Mutex<RoomState>>>>,
}

type ProtectedAppState = std::sync::LazyLock<Arc<AppState>>;

#[actix_web::get("/create-room")]
async fn create_room(req: actix_web::HttpRequest, stream: web::Payload, data: web::Data<&ProtectedAppState>) -> impl actix_web::Responder {
    // Before creating a new room, check if some rooms can be deleted
    let mutex_ok = match data.rooms.lock() {
        Ok(mut rooms) => {
            server_internal::remove_empty_rooms(&mut rooms);
            true
        },
        Err(_) => {
            println!("If we go in this branch, the server probably went through a lot of problems... All rooms will be deleted");
            false
        }
    };

    if !mutex_ok { // If the mutex was poisoned, reset the rooms
        data.rooms.clear_poison();
        *data.rooms.lock().unwrap() = HashMap::new();
    }

    let code = util::create_random_code();

    let test_info = PlayerInfo { name: String::from("John client 1") };
    let new_room = RoomState {
        game_state: game::get_initial_game_state(),
        host_player: Player::new(Some(test_info)),
        other_player: None,
        join_code: code.clone(),
        game_started: false,
        game_options: GameOptions::default(),
    };

    println!("Room creation request: {}. Now there are {} rooms active.", new_room.join_code, data.rooms.lock().unwrap().len() + 1);

    let room_in_arc = Arc::new(Mutex::new(new_room));

    data.rooms.lock().unwrap().insert(code.clone(), Arc::clone(&room_in_arc));
    
    let (response, connection) = server_internal::start_websocket(req, stream)?;
    server_internal::handle_player_connection(Arc::clone(&room_in_arc), true, connection).await.unwrap();
    server_internal::send_message(&mut room_in_arc.lock().unwrap().host_player, "room-code", &code);

    Ok::<HttpResponse, actix_web::Error>(response)
}

#[actix_web::get("/join-room/{room_code}")]
async fn join_room(req: actix_web::HttpRequest, stream: web::Payload, data: web::Data<&ProtectedAppState>, path: web::Path<String>) -> impl actix_web::Responder {
    let room_code = path.into_inner();

    if data.rooms.lock().unwrap().contains_key(&room_code) { // Room exists
        let room = Arc::clone(&data.rooms.lock().unwrap()[&room_code]);

        if room.lock().unwrap().other_player.is_some() { // Room already full
            return Ok::<HttpResponse, actix_web::Error>(HttpResponse::BadRequest().body("Room already full"));
        }
        
        println!("Player is joining room {}", room_code);

        let (response, connection) = server_internal::start_websocket(req, stream)?;

        let test_info = PlayerInfo { name: String::from("John client 2") };
        let mut player = Player::new(Some(test_info));
        player.ready_to_restart = true; // Player is immediately ready

        room.lock().unwrap().other_player = Some(player); // Add player to room
        server_internal::handle_player_connection(Arc::clone(&room), false, connection).await.unwrap(); // Start handling connection
        server_internal::send_message(&mut room.lock().unwrap().host_player, "other-player-connected", &()); // Tell the other player
        
        if room.lock().unwrap().host_player.ready_to_restart {
            game::check_for_restart_end(&mut room.lock().unwrap());
        }
        else {
            server_internal::send_message(&mut room.lock().unwrap().get_player(false), "wait-for-host", &());
        }

        Ok::<HttpResponse, actix_web::Error>(response)
    }
    else {
        Ok::<HttpResponse, actix_web::Error>(HttpResponse::NotFound().body("No room with this code"))
    }
}

#[actix_web::get("/reconnect/{which_player}/{room_code}")]
async fn reconnect(req: actix_web::HttpRequest, stream: web::Payload, data: web::Data<&ProtectedAppState>, path: web::Path<(u32, String)>) -> impl actix_web::Responder {
    let (which_player, room_code) = path.into_inner();
    let is_host_player = which_player == 0;

    println!("Reconnection of player {} in {}", which_player, room_code);

    if data.rooms.lock().unwrap().contains_key(&room_code) { // Room exists
        let room = Arc::clone(&data.rooms.lock().unwrap()[&room_code]);
        let mut locked_room = room.lock().unwrap();

        if !locked_room.player_exists(is_host_player) || locked_room.get_player(is_host_player).connection_alive {
            return Ok::<HttpResponse, actix_web::Error>(HttpResponse::BadRequest().body("Player not disconnected"));
        }
        
        let player = locked_room.get_player(is_host_player);
        let (response, connection) = server_internal::start_websocket(req, stream)?;

        player.connection_alive = true;
        player.last_ping_time = std::time::Instant::now();
        server_internal::handle_player_connection(Arc::clone(&room), is_host_player, connection).await.unwrap();

        Ok::<HttpResponse, actix_web::Error>(response)
    }
    else {
        Ok::<HttpResponse, actix_web::Error>(HttpResponse::NotFound().body("No room with this code"))
    }
}

#[actix_web::get("/ping")]
async fn ping() -> impl actix_web::Responder {
    Ok::<HttpResponse, actix_web::Error>(HttpResponse::Ok().body(""))
}

static APP_DATA: ProtectedAppState = std::sync::LazyLock::new(|| Arc::new(AppState { 
    rooms: Mutex::new(HashMap::new())
}));

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();

    println!("Starting backend server!");
    
    let address = if args.contains(&String::from("--localhost")) {
        println!("Serving on local network");
        "0.0.0.0:4268"
    }
    else {
        "[2a09:6847:fa10:1410::278]:4268"
    };

    actix_web::HttpServer::new(|| {
        actix_web::App::new()
        .app_data(actix_web::web::Data::new(&APP_DATA))
            .service(create_room)
            .service(join_room)
            .service(reconnect)
            .service(ping)
    })
    .bind(address)?
    .run()
    .await
}


impl RoomState {
    pub fn player_exists(&self, get_host_player: bool) -> bool {
        if get_host_player {
            true
        }
        else {
            self.other_player.is_some()
        }
    }

    pub fn get_player(&mut self, get_host_player: bool) -> &mut Player {
        if get_host_player {
            &mut self.host_player
        }
        else {
            self.other_player.as_mut().unwrap()
        }
    }

    pub fn do_for_all_players(&mut self, f: &dyn Fn(&mut Player, &mut Player) -> ()) {
        f(&mut self.host_player, self.other_player.as_mut().unwrap());
        f(self.other_player.as_mut().unwrap(), &mut self.host_player);
    }
}

impl Player {
    pub fn new(info: Option<PlayerInfo>) -> Player {
        Player { 
            player_info: info, 
            messages_to_send: Vec::new(),
            connection_alive: true,
            last_ping_time: std::time::Instant::now(),
            past_words: Vec::with_capacity(MAX_WORD_COUNT as usize),
            typed_word_this_turn: None,
            letter_sabotaged_this_turn: None,
            ready_to_restart: false,
        }
    }
}

impl GameOptions {
    pub fn default() -> GameOptions {
        GameOptions { lang: Language::English, mode: Mode::Normal, attacks: AttackMode::OncePerTurn }
    }
}
