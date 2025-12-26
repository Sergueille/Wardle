
use crate::Player;
use crate::RoomState;
use crate::game;

use actix_web::HttpResponse;
use actix_web::web;
use futures_util::TryStreamExt;
use std::sync::{Arc, Mutex};

const PLAYER_CONNECTION_LOOP_INTERVAL: u64 = 100; // ms

/// Maximum duration before a player is disconnected it they sent no pings
const PLAYER_SILENCE_MAX_DURATION: u64 = 10000; // ms


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

    // Connection loop (send messages, check for pings...)
    actix_web::rt::spawn(async move { loop {
        let mut messages_to_send = Vec::new();

        { // Block where the room is locked
            let mut room_ref = cloned_arc.lock().unwrap();
            let player = room_ref.get_player(host_player);
            
            if !player.connection_alive { // End loop if disconnected
                break;
            }

            if std::time::Instant::now().duration_since(player.last_ping_time).as_millis() > PLAYER_SILENCE_MAX_DURATION as u128 { // Disconnect if no pings
                player.connection_alive = false;
                let _ = connection.session.close(None).await;
                println!("A player is disconnected because they sent no pings");
                break;
            }

            // Get the list of messages to send from the room
            std::mem::swap(&mut messages_to_send, &mut room_ref.get_player(host_player).messages_to_send);
        }

        // Send the messages
        let mut all_ok = true;
        messages_to_send.reverse();
        while messages_to_send.len() > 0 {
            let top = messages_to_send.pop().unwrap();
            let sent_res = connection.session.text(top).await;
            if sent_res.is_err() {
                cloned_arc.lock().unwrap().get_player(host_player).connection_alive = false;
                all_ok = false;
                break; // Stop if connection closed
            }
        }

        if !all_ok { // If some messages couldn't be sent correctly, put them back in the list
            messages_to_send.reverse();
            let mut room_ref = cloned_arc.lock().unwrap();
            std::mem::swap(&mut messages_to_send, &mut room_ref.get_player(host_player).messages_to_send);
            break;
        }

        actix_web::rt::time::sleep(std::time::Duration::from_millis(PLAYER_CONNECTION_LOOP_INTERVAL)).await;
    }});

    let room_ref = Arc::clone(&room);

    // Listen for messages
    actix_web::rt::spawn(async move { loop {
        if !room_ref.lock().unwrap().get_player(host_player).connection_alive {
            break; // End loop if disconnected
        }

        match connection.stream.try_next().await {
            Ok(Some(actix_ws::AggregatedMessage::Text(text))) => {
                room.lock().unwrap().get_player(host_player).last_ping_time = std::time::Instant::now();

                match handle_one_message_internal(Arc::clone(&room_ref), &text, host_player) {
                    Ok(()) => {},
                    Err(msg) => { log::error!("{}", msg); }
                }
            },
            Ok(Some(actix_ws::AggregatedMessage::Ping(_))) => {
                // TODO
            },
            _ => {
                room.lock().unwrap().get_player(host_player).connection_alive = false;
                break;
            }
        }
    }});

    Ok(())
}

fn handle_one_message_internal(room: Arc<Mutex<RoomState>>, text: &str, is_host: bool) -> Result<(), String> {
    match serde_json::de::from_str::<serde_json::Value>(text).map_err(|err| err.to_string())? {
        serde_json::Value::Object(o) => {
            let msg_type = crate::util::get_json_str(&o, "type")?;
            let msg_content = crate::util::get_json_obj(&o, "content")?;

            game::handle_one_message(&mut room.lock().unwrap(), msg_type, msg_content, is_host)
        },
        _ => {
            Err(format!("Websocket message is not an object"))
        }
    }

}

pub fn remove_empty_rooms(rooms: &mut std::collections::HashMap<String, Arc<Mutex<RoomState>>>) {
    rooms.retain(|_, room| {
        match room.lock() {
            Ok(room_ref) => {
                // Room is still alive is at least one player have a valid connection
                room_ref.host_player.connection_alive || (room_ref.other_player.is_some() && room_ref.other_player.as_ref().unwrap().connection_alive)
            },
            Err(_) => {
                // The mutex is probably poisoned, so just remove the room and act as if nothing happened
                false
            },
        }
    });
}
