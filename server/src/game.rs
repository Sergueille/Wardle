
use crate::{server_internal::send_message, *};

type JsonMap = serde_json::Map<String, serde_json::Value>;

pub const MESSAGE_TYPE_CURSORS: &str = "cursors";
pub const MESSAGE_TYPE_NEW_PLAYER: &str = "new_player";
pub const MESSAGE_TYPE_PLAYER_LIST: &str = "player_list";

pub const WORD_LENGTH: u64 = 5;
pub const MAX_WORD_COUNT: u64 = 6;

pub fn get_initial_game_state() -> GameState {
    let word_to_guess = util::get_random_secret_word();
    println!("Word to guess is {}", word_to_guess);

    GameState {
        current_turn: -1,
        current_phase: GamePhase::Typing,
        word_to_guess,
    }
}

pub fn game_start(room: &mut RoomState) {
    room.game_started = true;
    room.do_for_all_players(&|p, _| p.ready_to_restart = false);
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

pub fn check_for_type_end(room: &mut RoomState) {
    // Check that both plater entered a word
    if room.get_player(false).typed_word_this_turn.is_none() 
    || room.get_player(true).typed_word_this_turn.is_none() {
        return;
    }

    // Go to next phase
    room.game_state.current_phase = GamePhase::Sabotaging;

    // Send other words
    #[derive(serde::Serialize)]
    struct Msg<'a> {
        word: &'a str,
        who_wins: String, 
    }

    let host_word = room.host_player.typed_word_this_turn.clone().unwrap();
    let other_word = room.other_player.as_ref().unwrap().typed_word_this_turn.clone().unwrap();
    let mut host_msg = Msg {
        word: &other_word,
        who_wins: String::from("none"),
    };
    let mut other_msg = Msg {
        word: &host_word,
        who_wins: String::from("none"),
    };

    if host_word == room.game_state.word_to_guess {
        host_msg.who_wins = String::from("you");
        other_msg.who_wins = String::from("other");
    }
    else if other_word == room.game_state.word_to_guess {
        host_msg.who_wins = String::from("other");
        other_msg.who_wins = String::from("you");
    }

    send_message(&mut room.host_player, "other-player-word", &host_msg);
    send_message(room.other_player.as_mut().unwrap(), "other-player-word", &other_msg);

    let someone_win = host_word == room.game_state.word_to_guess || other_word == room.game_state.word_to_guess;
    if someone_win {
        on_game_end(room);
        return;
    }

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
}

pub fn on_game_end(room: &mut RoomState) {
    // Just reset the room for it to be ready for restart
    room.game_state = get_initial_game_state();
    room.game_state.current_phase = GamePhase::Restarting;
}

pub fn check_for_sabotage_end(room: &mut RoomState) {
    // Check that both player sabotaged
    if room.get_player(false).letter_sabotaged_this_turn.is_none() 
    || room.get_player(true).letter_sabotaged_this_turn.is_none() {
        return;
    }

    // Send hints
    let word_to_guess = room.game_state.word_to_guess.clone();
    room.do_for_all_players(&|player: &mut Player, other| {
        let hints = hints::get_hints(
            &word_to_guess, 
            &player.typed_word_this_turn.as_ref().unwrap(), 
            other.letter_sabotaged_this_turn.unwrap() as usize
        );

        send_message(player, "word-hints", &hints::get_hints_strings(hints));    
    });

    start_turn(room); // Next turn
}

pub fn check_for_restart_end(room: &mut RoomState) {
    // Check that both player restarted
    if !room.get_player(false).ready_to_restart 
    || !room.get_player(true).ready_to_restart {
        return;
    }

    room.do_for_all_players(&|p, _| { send_message(p, "restart", &()); });

    game_start(room);
}

pub fn handle_one_message(room: &mut RoomState, msg_type: &str, msg_contents: &JsonMap, is_host: bool) -> Result<(), String> {
    if !room.game_started && msg_type != "ping" {
        return Err(String::from("Message sent before game started")); // Do nothing if room hasn't even started
    }
    
    match msg_type {
        "ping" => {
            // Nothing to do
        },
        "word" => {
            if room.game_state.current_phase != GamePhase::Typing { return Err(String::from("Wrong phase")); }

            let word = crate::util::get_json_str(msg_contents, "word").unwrap();

            if util::is_valid_word(word) {   
                room.get_player(is_host).typed_word_this_turn = Some(String::from(word));
                check_for_type_end(room);
            }
            else {
                send_message(room.get_player(is_host), "word-rejected", &());
            }
        },
        "sabotage" => {
            if room.game_state.current_phase != GamePhase::Sabotaging { return Err(String::from("Wrong phase")); }

            let id = crate::util::get_json_number(msg_contents, "id").unwrap();
            room.get_player(is_host).letter_sabotaged_this_turn = Some(id.as_u64().unwrap()); // TODO: check that the number is between 0 and 5
            check_for_sabotage_end(room)
        },
        "restart-ready" => {
            if room.game_state.current_phase != GamePhase::Restarting { return Err(String::from("Wrong phase")); }
            room.get_player(is_host).ready_to_restart = true;
            check_for_restart_end(room);
        },
        _ => {
            return Err(format!("Unknown message type {}", msg_type));
        }
    }
    
    return Ok(());
}
