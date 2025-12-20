
#![allow(dead_code)]

mod util;
mod game;
mod server_internal;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use actix_web::HttpResponse;
use actix_web::web;

struct GameState {
    // TODO
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
    messages_to_send: Vec<String>
}

struct RoomState {
    game_state: GameState,
    host_player: Player,
    other_player: Option<Player>,
    join_code: String,
}

struct AppState {
    rooms: Mutex<HashMap<String, Arc<Mutex<RoomState>>>>,
}

type ProtectedAppState = std::sync::LazyLock<Arc<AppState>>;

#[actix_web::get("/create-room")]
async fn create_room(req: actix_web::HttpRequest, stream: web::Payload, data: web::Data<&ProtectedAppState>) -> impl actix_web::Responder {
    let code = util::create_random_code();

    let test_info = PlayerInfo { name: String::from("John client 1") };
    let new_room = RoomState {
        game_state: GameState { 

        },
        host_player: Player {
            player_info: Some(test_info),
            messages_to_send: Vec::new(),
        },
        other_player: None,
        join_code: code.clone()
    };

    println!("Room creation request: {}", new_room.join_code);

    let room_in_arc = Arc::new(Mutex::new(new_room));

    data.rooms.lock().unwrap().insert(code.clone(), Arc::clone(&room_in_arc));
    
    let (response, connection) = server_internal::start_websocket(req, stream)?;
    game::start_room_loop(Arc::clone(&room_in_arc));
    server_internal::handle_player_connection(Arc::clone(&room_in_arc), true, connection).await.unwrap();
    server_internal::send_message(&mut room_in_arc.lock().unwrap().host_player, "room-code", &code);

    Ok::<HttpResponse, actix_web::Error>(response)
}

#[actix_web::get("/join-room/{room_code}")]
async fn join_room(req: actix_web::HttpRequest, stream: web::Payload, data: web::Data<&ProtectedAppState>, path: web::Path<String>) -> impl actix_web::Responder {
    let room_code = path.into_inner();

    let rooms = data.rooms.lock().unwrap();

    if rooms.contains_key(&room_code) { // Room exists
        let room = &rooms[&room_code];

        if room.lock().unwrap().other_player.is_some() { // Room already full
            return Ok::<HttpResponse, actix_web::Error>(HttpResponse::BadRequest().body("Room already full"));
        }
        
        println!("Player is joining room {}", room_code);

        let (response, connection) = server_internal::start_websocket(req, stream)?;

        let test_info = PlayerInfo { name: String::from("John client 2") };
        let player = Player { 
            player_info: Some(test_info), 
            messages_to_send: Vec::new()
        };

        room.lock().unwrap().other_player = Some(player); // Add player to room
        server_internal::handle_player_connection(Arc::clone(room), false, connection).await.unwrap(); // Start handling connection
        server_internal::send_message(&mut room.lock().unwrap().host_player, "other-player-connected", &()); // Tell the other player

        Ok::<HttpResponse, actix_web::Error>(response)
    }
    else {
        Ok::<HttpResponse, actix_web::Error>(HttpResponse::NotFound().body("No room with this code"))
    }
}

static APP_DATA: ProtectedAppState = std::sync::LazyLock::new(|| Arc::new(AppState { 
    rooms: Mutex::new(HashMap::new())
}));

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    actix_web::HttpServer::new(|| {
        actix_web::App::new()
        .app_data(actix_web::web::Data::new(&APP_DATA))
            .service(create_room)
            .service(join_room)
    })
    .bind(("localhost", 4268))?
    .run()
    .await
}


impl RoomState {
    pub fn get_player(&mut self, get_host_player: bool) -> &mut Player {
        if get_host_player {
            &mut self.host_player
        }
        else {
            self.other_player.as_mut().unwrap()
        }
    }
}
