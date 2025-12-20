
use crate::Player;
use crate::RoomState;
use crate::game;

use actix_web::HttpResponse;
use actix_web::web;
use futures_util::TryStreamExt;
use std::sync::{Arc, Mutex};

pub fn create_message_text<T>(message_type: &str, message_contents: &T) -> String where T : serde::Serialize {
    let serialized_contents = serde_json::to_string(message_contents).unwrap();
    format!("{{\"type\":\"{}\",\"content\":{}}}", message_type, serialized_contents)
}

pub fn send_message<T>(player: &mut Player, message_type: &str, message_contents: &T) where T : serde::Serialize {
    let message_text = create_message_text(message_type, message_contents);
    player.messages_to_send.push(message_text.clone());
}

pub fn send_message_to_both_players<T>(room: &mut RoomState, message_type: &str, message_contents: &T) where T : serde::Serialize {
    let message_text = create_message_text(message_type, message_contents);
    
    room.host_player.messages_to_send.push(message_text.clone());
    room.other_player.as_mut().map(|other_player| other_player.messages_to_send.push(message_text.clone()));
}

pub fn start_websocket(req: actix_web::HttpRequest, stream: web::Payload) 
    -> Result<(HttpResponse, crate::SocketConnection), actix_web::Error> {
    let (res, session, stream) = actix_ws::handle(&req, stream)?;

    let stream = stream
        .aggregate_continuations()
        .max_continuation_size(2_usize.pow(20));

    Ok((res, crate::SocketConnection { session, stream }))
}


pub async fn handle_player_connection(room: Arc<Mutex<RoomState>>, host_player: bool, mut connection: crate::SocketConnection) -> Result<(), actix_ws::Closed> {
    let cloned_arc = Arc::clone(&room);

    // Send messages
    actix_web::rt::spawn(async move { loop {
        let mut messages = Vec::new();
        
        { // Get the list of messages to send
            let mut room_ref = cloned_arc.lock().unwrap();
            std::mem::swap(&mut messages, &mut room_ref.get_player(host_player).messages_to_send);
        }

        for m in messages {
            let sent_res = connection.session.text(m.clone()).await;
            if sent_res.is_err() { break } // Stop if connection closed
        }
        
        actix_web::rt::time::sleep(std::time::Duration::from_millis(100)).await;
    }});

    let room_ref = Arc::clone(&room);

    // Listen for messages
    actix_web::rt::spawn(async move { loop {
        match connection.stream.try_next().await {
            Ok(Some(actix_ws::AggregatedMessage::Text(text))) => {
                match handle_one_message_internal(Arc::clone(&room_ref), &text) {
                    Ok(()) => {},
                    Err(msg) => { log::error!("{}", msg); }
                }
            },
            Ok(Some(actix_ws::AggregatedMessage::Ping(msg))) => {
                // TODO
            },
            _ => {
                break;
            }
        }
    }});

    Ok(())
}

fn handle_one_message_internal(room: Arc<Mutex<RoomState>>, text: &str) -> Result<(), String> {
    match serde_json::de::from_str::<serde_json::Value>(text).map_err(|err| err.to_string())? {
        serde_json::Value::Object(o) => {
            let msg_type = crate::util::get_json_str(&o, "type")?;
            let msg_content = crate::util::get_json_obj(&o, "content")?;

            game::handle_one_message(room, msg_type, msg_content)
        },
        _ => {
            Err(format!("Websocket message is not an object"))
        }
    }

}

