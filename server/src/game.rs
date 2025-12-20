
use crate::*;
use std::sync::{Arc, Mutex};

type JsonMap = serde_json::Map<String, serde_json::Value>;

pub const MESSAGE_TYPE_CURSORS: &str = "cursors";
pub const MESSAGE_TYPE_NEW_PLAYER: &str = "new_player";
pub const MESSAGE_TYPE_PLAYER_LIST: &str = "player_list";


pub fn start_room_loop(room: Arc<Mutex<RoomState>>) {
    actix_web::rt::spawn(async move {
        loop {

            // TODO

            actix_web::rt::time::sleep(std::time::Duration::from_millis(20)).await;
        }  

        // TODO: Stop looping if players left room
    });
}

pub fn handle_one_message(room: Arc<Mutex<RoomState>>, msg_type: &str, msg_contents: &JsonMap) -> Result<(), String> {
    match msg_type {
        "ping" => {
            Ok(()) // Nothing to do
        },
        _ => {
            return Err(format!("Unknown message type {}", msg_type));
        }
    }
}