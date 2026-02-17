
use crate::{server_internal::send_message, *};

type JsonMap = serde_json::Map<String, serde_json::Value>;

pub const MESSAGE_TYPE_CURSORS: &str = "cursors";
pub const MESSAGE_TYPE_NEW_PLAYER: &str = "new_player";
pub const MESSAGE_TYPE_PLAYER_LIST: &str = "player_list";

pub const WORD_LENGTH: u64 = 5;
pub const MAX_WORD_COUNT: u64 = 6;

pub fn get_initial_game_state() -> GameState {
    GameState {
        current_turn: -1,
        current_phase: GamePhase::Typing,
        word_to_guess: None,
    }
}

pub fn game_start(room: &mut RoomState) {
    room.game_started = true;
    room.do_for_all_players(&|p, _| p.ready_to_restart = false);

    // Pick a new word to guess
    let word_to_guess = util::get_random_secret_word(room.game_options.language);
    println!("Word to guess is {}", word_to_guess);
    room.game_state.word_to_guess = Some(word_to_guess);

    send_options(room.game_options.clone(), room.get_player(false));
    start_turn(room);
}

pub fn start_turn(room: &mut RoomState) {
    room.game_state.current_turn += 1;
    room.game_state.current_phase = GamePhase::Typing;
    room.host_player.typed_word_this_turn = None;
    room.other_player.as_mut().unwrap().typed_word_this_turn = None;
    room.host_player.letter_sabotaged_this_turn = None;
    room.other_player.as_mut().unwrap().letter_sabotaged_this_turn = None;
}

/// Do things if both player typed their word. Returns wether both player typed their words (returns true is there is a victory)
pub fn check_for_type_end(room: &mut RoomState) -> bool {
    // Check for victory
    let player_a = &mut room.host_player;
    let player_b = room.other_player.as_mut().unwrap();
    let word_to_guess = &room.game_state.word_to_guess.as_ref().unwrap();

    let a_wins = handle_victory_condition(player_a, player_b, word_to_guess);
    let b_wins = handle_victory_condition(player_b, player_a, word_to_guess);

    if a_wins || b_wins { // Game finished 
        on_game_end(room);
        return true;
    } 

    // Before handling turn end, check that both plater entered a word
    if room.get_player(false).typed_word_this_turn.is_none() 
    || room.get_player(true).typed_word_this_turn.is_none() {
        return false;
    }

    // Go to next phase
    room.game_state.current_phase = GamePhase::Sabotaging;

    // Send other words
    #[derive(serde::Serialize)]
    struct Msg<'a> {
        word: &'a str,
    }

    let host_word = room.host_player.typed_word_this_turn.clone().unwrap();
    let other_word = room.other_player.as_ref().unwrap().typed_word_this_turn.clone().unwrap();
    let host_msg = Msg {
        word: &other_word,
    };
    let other_msg = Msg {
        word: &host_word,
    };

    send_message(&mut room.host_player, "other-player-word", &host_msg);
    send_message(room.other_player.as_mut().unwrap(), "other-player-word", &other_msg);

    let was_last_guess = room.game_state.current_turn == MAX_WORD_COUNT as i64 - 1; // Was this the last possible guess for this game?
    let solution = room.game_state.word_to_guess.clone();

    room.do_for_all_players(&|player, _| {
        player.past_words.push(player.typed_word_this_turn.as_ref().unwrap().clone());

        if was_last_guess {
            send_message(player, "solution", &solution);
        }
    });

    if was_last_guess { 
        on_game_end(room);
    }

    return true;
}

// Handles victory. Returns wether `player` won the game
pub fn handle_victory_condition(player: &mut Player, other: &mut Player, word_to_guess: &str) -> bool {
    match &player.typed_word_this_turn {
        Some(w) => {
            if *w == word_to_guess {
                match &other.typed_word_this_turn {
                    Some(other_w) => send_message(player, "you-win", &other_w),
                    None => send_message(player, "you-win", &()),
                }
                
                send_message(other, "other-player-win", &word_to_guess);
                true
            }
            else {
                false
            }
        },
        None => false,
    }
}

pub fn on_game_end(room: &mut RoomState) {
    // Just reset the room for it to be ready for restart
    room.game_state = get_initial_game_state();
    room.game_state.current_phase = GamePhase::Restarting;
}

pub fn check_for_sabotage_end(room: &mut RoomState) -> bool {
    // Check that both player sabotaged
    if room.get_player(false).letter_sabotaged_this_turn.is_none() 
    || room.get_player(true).letter_sabotaged_this_turn.is_none() {
        return false;
    }

    // Send hints
    let word_to_guess = room.game_state.word_to_guess.clone();
    room.do_for_all_players(&|player: &mut Player, other| {
        let hints = hints::get_hints(
            word_to_guess.as_ref().unwrap(), 
            &player.typed_word_this_turn.as_ref().unwrap(), 
            other.letter_sabotaged_this_turn.unwrap() as usize
        );

        send_message(player, "word-hints", &hints::get_hints_strings(hints));    
    });

    start_turn(room); // Next turn

    return true;
}

pub fn check_for_restart_end(room: &mut RoomState) {
    // Check that both player restarted
    if !room.player_exists(false) || !room.get_player(false).ready_to_restart 
    || !room.get_player(true).ready_to_restart {
        return;
    }

    room.do_for_all_players(&|p, _| { send_message(p, "restart", &()); });

    game_start(room);
}

pub fn handle_one_message(room: &mut RoomState, msg_type: &str, msg_contents: &JsonMap, is_host: bool) -> Result<(), String> {    
    match msg_type {
        "ping" => {
            // Nothing to do
        },
        "word" => {
            if room.game_state.current_phase != GamePhase::Typing { return Err(String::from("Wrong phase")); }

            let word = crate::util::get_json_str(msg_contents, "word").unwrap();

            if util::is_valid_word(word, room.game_options.language) {   
                room.get_player(is_host).typed_word_this_turn = Some(String::from(word));
                let ended = check_for_type_end(room);

                if !ended { // Tell the other player
                    send_message(room.get_player(!is_host), "other-player-is-done", &());
                }
            }
            else {
                send_message(room.get_player(is_host), "word-rejected", &());
            }
        },
        "sabotage" => {
            if room.game_state.current_phase != GamePhase::Sabotaging { return Err(String::from("Wrong phase")); }

            let id = crate::util::get_json_number(msg_contents, "id").unwrap();
            room.get_player(is_host).letter_sabotaged_this_turn = Some(id.as_u64().unwrap()); // TODO: check that the number is between 0 and 5
            let ended = check_for_sabotage_end(room);

            // Tell the other player
            if !ended { send_message(room.get_player(!is_host), "other-player-is-done", &()); }
        },
        "restart-ready" => {
            if room.game_state.current_phase != GamePhase::Restarting && room.game_started { return Err(String::from("Wrong phase")); }
            room.get_player(is_host).ready_to_restart = true;
            check_for_restart_end(room);
        },
        "game-options" => {
            if room.game_started && room.game_state.current_phase != GamePhase::Restarting { return Err(String::from("Game in progress")); }
            room.game_options = serde_json::from_value(msg_contents.get("options").unwrap().clone()).expect("Invalid option format");

            if room.game_state.current_phase == GamePhase::Restarting {
                // Tell the other player only if in restart phase. 
                // If the host changes the option for the first time, the options will be sent when the game starts
                send_options(room.game_options.clone(), room.get_player(!is_host));
            }
        },
        _ => {
            return Err(format!("Unknown message type {}", msg_type));
        }
    }
    
    return Ok(());
}


pub fn send_options(options: GameOptions, player: &mut Player) {
    #[derive(serde::Serialize, Clone)]
    struct MessageType {
        options: GameOptions,
    }
    let current_options = MessageType { options: options };
    send_message(player, "game-options", &current_options);
}

