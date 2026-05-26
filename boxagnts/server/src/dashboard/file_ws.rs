use axum::extract::WebSocketUpgrade;
use axum::extract::ws::{Message, WebSocket};
use axum::response::IntoResponse;
use futures::StreamExt;

use boxagnts_gateway::api::fs_events;

async fn fs_changes_socket(mut socket: WebSocket) {
    let mut rx = fs_events::subscribe();

    loop {
        tokio::select! {
            msg = rx.recv() => {
                match msg {
                    Ok(event) => {
                        let text = match serde_json::to_string(&event) {
                            Ok(v) => v,
                            Err(_) => continue,
                        };

                        if socket.send(Message::Text(text.into())).await.is_err() {
                            break;
                        }
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => {
                        continue;
                    }
                    Err(_) => {
                        break;
                    }
                }
            }
            incoming = socket.next() => {
                match incoming {
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Ok(Message::Ping(v))) => {
                        if socket.send(Message::Pong(v)).await.is_err() {
                            break;
                        }
                    }
                    Some(Ok(_)) => {}
                    Some(Err(_)) => break,
                }
            }
        }
    }
}


pub async fn handle_websocket(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(fs_changes_socket)
}