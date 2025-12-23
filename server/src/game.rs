
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
        word_to_guess: String::from("LOOSE"), // TODO
    }
}

pub fn game_start(room: &mut RoomState) {
    room.game_started = true;
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
    send_message(&mut room.host_player, "other-player-word", &room.other_player.as_ref().unwrap().typed_word_this_turn.clone().unwrap());
    send_message(room.other_player.as_mut().unwrap(), "other-player-word", &room.host_player.typed_word_this_turn.clone().unwrap());

    room.do_for_all_players(&|player, _other| {
        player.past_words.push(player.typed_word_this_turn.as_ref().unwrap().clone());
    });
}

pub fn check_for_sabotage_end(room: &mut RoomState) {
    // Check that both plater sabotaged
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
            room.get_player(is_host).typed_word_this_turn = Some(String::from(word));
            check_for_type_end(room);
        },
        "sabotage" => {
            if room.game_state.current_phase != GamePhase::Sabotaging { return Err(String::from("Wrong phase")); }

            let id = crate::util::get_json_number(msg_contents, "id").unwrap();
            room.get_player(is_host).letter_sabotaged_this_turn = Some(id.as_u64().unwrap()); // TODO: check that the number is between 0 and 5
            check_for_sabotage_end(room)
        },
        _ => {
            return Err(format!("Unknown message type {}", msg_type));
        }
    }
    
    return Ok(());
}
