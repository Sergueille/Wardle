
use crate::*;

type JsonMap = serde_json::Map<String, serde_json::Value>;

pub const MESSAGE_TYPE_CURSORS: &str = "cursors";
pub const MESSAGE_TYPE_NEW_PLAYER: &str = "new_player";
pub const MESSAGE_TYPE_PLAYER_LIST: &str = "player_list";

pub fn handle_one_message(room: &mut RoomState, msg_type: &str, msg_contents: &JsonMap) -> Result<(), String> {
    match msg_type {
        "ping" => {
            Ok(()) // Nothing to do
        },
        _ => {
            return Err(format!("Unknown message type {}", msg_type));
        }
    }
}
