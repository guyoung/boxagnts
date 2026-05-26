use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use axum::extract::State as AxumState;
use axum::extract::WebSocketUpgrade;
use axum::response::Response;
use futures_util::{FutureExt, SinkExt, StreamExt};
use serde::Deserialize;
use serde_json::{json, Value};
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;

#[derive(Clone)]
pub struct ChatWSAppState {
    // instance_id -> websocket sender
    pub ws_instances: Arc<Mutex<HashMap<String, tokio::sync::mpsc::Sender<String>>>>,

    // session_id -> cancellation token
    pub running_queries: Arc<Mutex<HashMap<String, CancellationToken>>>,
}

#[derive(Debug, Deserialize)]
pub struct ExecutionRequest {
    pub project_path: Option<String>,
    pub prompt: Option<String>,
    pub model: Option<String>,
    pub session_id: Option<String>,
    pub command_type: String, // "execute" | "cancel"
}

/// WebSocket handler for execution with streaming output
pub async fn handle_websocket(
    ws: WebSocketUpgrade,
    AxumState(state): AxumState<ChatWSAppState>,
) -> Response {
    ws.on_upgrade(move |socket| process(socket, state))
}

async fn process(socket: WebSocket, state: ChatWSAppState) {
    let (mut sender, mut receiver) = socket.split();
    let instance_id = uuid::Uuid::new_v4().to_string();

    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(100);

    {
        let mut instances = state.ws_instances.lock().await;
        instances.insert(instance_id.clone(), tx);
    }

    let forward_task = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            if sender.send(Message::Text(message.into())).await.is_err() {
                break;
            }
        }
    });



    while let Some(msg) = receiver.next().await {
        // println!("[TRACE] Received WebSocket message: {:?}", msg);

        match msg {
            Ok(Message::Text(text)) => match serde_json::from_str::<ExecutionRequest>(&text) {
                Ok(request) => {
                    match request.command_type.as_str() {
                        "chat_execute" => {
                            let instance_id_clone = instance_id.clone();
                            let state_clone = state.clone();

                            let session_id = request
                                .session_id
                                .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

                            let project_path = request.project_path.unwrap_or_default();
                            let prompt = request.prompt.unwrap_or_default();
                            let model = request.model.unwrap_or_default();

                            tokio::spawn(async move {
                                let result = execute_command(
                                    project_path,
                                    prompt,
                                    model,
                                    session_id.clone(),
                                    state_clone.clone(),
                                    instance_id_clone.clone(),
                                )
                                    .await;

                                let completion_msg = match result {
                                    Ok(result) => json!({
                                        "type": "completion",
                                        "session_id": session_id,
                                        "result": result
                                    }),
                                    Err(e) => json!({
                                        "type": "error",
                                        "session_id": session_id,
                                        "message": e.to_string()
                                    }),
                                };

                                send_to_instance(
                                    state_clone.clone(),
                                    instance_id_clone.clone(),
                                    completion_msg.to_string(),
                                )
                                    .await;
                            });
                        }

                        "chat_execute_cancel" => {
                            let session_id = match request.session_id {
                                Some(id) => id,
                                None => {
                                    let error_msg = json!({
                                        "type": "error",
                                        "message": "cancel requires session_id"
                                    });

                                    send_to_instance(
                                        state.clone(),
                                        instance_id.clone(),
                                        error_msg.to_string(),
                                    )
                                        .await;
                                    continue;
                                }
                            };

                            let cancelled =
                                cancel_running_query(state.clone(), session_id.clone()).await;

                            let resp = if cancelled {
                                json!({
                                    "type": "cancel_sent",
                                    "session_id": session_id
                                })
                            } else {
                                json!({
                                    "type": "error",
                                    "message": format!("no running session found: {}", session_id)
                                })
                            };

                            send_to_instance(state.clone(), instance_id.clone(), resp.to_string())
                                .await;
                        }

                        _ => {
                            let error_msg = json!({
                                "type": "error",
                                "message": format!("Unknown command type: {}", request.command_type)
                            });

                            send_to_instance(
                                state.clone(),
                                instance_id.clone(),
                                error_msg.to_string(),
                            )
                                .await;
                        }
                    }
                }
                Err(e) => {
                    let error_msg = json!({
                        "type": "error",
                        "message": format!("Failed to parse request: {}", e)
                    });

                    send_to_instance(state.clone(), instance_id.clone(), error_msg.to_string())
                        .await;
                }
            },

            Ok(Message::Close(_)) => {
                break;
            }

            Ok(_other) => {

            }

            Err(e) => {
                println!("[TRACE] Error receiving WebSocket message: {}", e);
                break;
            }
        }
    }

    {
        let mut instances = state.ws_instances.lock().await;
        instances.remove(&instance_id);
    }

    forward_task.abort();
}

async fn execute_command(
    _project_path: String,
    prompt: String,
    model: String,
    session_id: String,
    state: ChatWSAppState,
    instance_id: String,
) -> anyhow::Result<Value> {
    send_to_instance(
        state.clone(),
        instance_id.clone(),
        json!({
            "type": "start",
            "session_id": session_id,
            "message": "Starting execution..."
        })
            .to_string(),
    )
        .await;

    let state_clone = state.clone();
    let session_id_clone = session_id.clone();
    let instance_id_clone = instance_id.clone();

    let state_2_clone = state.clone();
    let session_id_2_clone = session_id.clone();
    let instance_id_2_clone = instance_id.clone();

    let handle = boxagnts_gateway::api::chat::execute(
        prompt,
        Some(model.clone()),
        Some(session_id.clone()),
        move |val| {
            let state_clone = state_clone.clone();
            let instance_id_clone = instance_id_clone.clone();
            let session_id_clone = session_id_clone.clone();

            async move {
                let message = json!({
                    "type": "output",
                    "session_id": &session_id_clone,
                    "content": val
                })
                    .to_string();

                send_to_instance(state_clone, instance_id_clone, message).await;
            }
                .boxed()
        },
        move |val| {
            let state_2_clone = state_2_clone.clone();
            let instance_id_2_clone = instance_id_2_clone.clone();
            let session_id_2_clone = session_id_2_clone.clone();

            async move {
                let message = json!({
                    "type": "output",
                    "session_id": &session_id_2_clone,
                    "content": val
                })
                    .to_string();

                send_to_instance(state_2_clone, instance_id_2_clone, message).await;
            }
                .boxed()
        },
    )
        .await?;

    {
        let mut running = state.running_queries.lock().await;
        running.insert(session_id.clone(), handle.cancellation_token());
    }

    let result = handle.await_result().await;

    {
        let mut running = state.running_queries.lock().await;
        running.remove(&session_id);
    }

    result
}

async fn cancel_running_query(state: ChatWSAppState, session_id: String) -> bool {
    let running = state.running_queries.lock().await;
    if let Some(token) = running.get(&session_id) {
        token.cancel();
        true
    } else {
        false
    }
}

async fn send_to_instance(state: ChatWSAppState, instance_id: String, message: String) {
    let sender_opt = {
        let instances = state.ws_instances.lock().await;
        instances.get(&instance_id).cloned()
    };

    if let Some(sender) = sender_opt {
        if let Err(e) = sender.send(message).await {
            println!("[TRACE] Failed to send message: {}", e);
        }
    } else {
        println!(
            "[TRACE] Session {} not found in WebSocket instances",
            instance_id
        );
    }
}
