use crate::server::types::AppState;
use crate::server::websocket::utils::{check_username, create_bot_msg};
use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use axum::{
    extract::{ws::WebSocketUpgrade, State},
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};

// fn to handle websocket connections.
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, state))
}

use tokio::sync::{broadcast, mpsc};

// This function deals with a single websocket connection, i.e., a single
// connected client / user, for which we will spawn two independent tasks (for
// receiving / sending chat messages).
async fn websocket(stream: WebSocket, state: Arc<AppState>) {
    //
    // Mistral
    // Load unique conversation log per user.
    let mut conversation_history: Vec<String> = vec![];

    // By splitting, we can send and receive at the same time.
    let (mut sender_sink, mut receiver_stream) = stream.split();
    let (sender_mpsc, mut receiver_mpsc) = mpsc::channel::<String>(16);

    tokio::spawn(async move {
        while let Some(message) = receiver_mpsc.recv().await {
            if sender_sink.send(message.into()).await.is_err() {
                break;
            }
        }
    });

    // Username gets set in the receive loop, if it's valid.
    let mut username = String::new();
    // Loop until a text message is found.
    while let Some(Ok(message)) = receiver_stream.next().await {
        if let Message::Text(name) = message {
            // If username that is sent by client is not taken, fill username string.
            check_username(&state, &mut username, &name);

            // If not empty we want to quit the loop else we want to quit function.
            if !username.is_empty() {
                break;
            } else {
                // Only send our client that username is taken.
                let _ = sender_mpsc
                    .send(String::from("Username already taken."))
                    .await;

                return;
            }
        }
    }

    // We subscribe *before* sending the "joined" message, so that we will also
    // display it to our client.
    let mut receiver_subscribed: broadcast::Receiver<String> = state.broadcast_sender.subscribe();

    // Now send the "joined" message to all subscribers.
    let join_msg = format!("{} joined.", username);
    tracing::debug!("{}", join_msg);
    let _ = state.broadcast_sender.send(join_msg);

    // Clone things we want to pass (move) to the receiving task.
    // forward to mpsc from receiver_subscribed
    let sender_clone = sender_mpsc.clone();

    // Spawn the first task that will receive broadcast messages and send text
    // messages over the websocket to our client.
    let mut send_join_handle = tokio::spawn(async move {
        while let Ok(msg_out) = receiver_subscribed.recv().await {
            // In any websocket error, break loop.

            if sender_clone.send(msg_out.to_string()).await.is_err() {
                break;
            }
        }
    });

    // Clone things we want to pass (move) to the receiving task.
    let broadcast_sender_clone = state.broadcast_sender.clone();
    let sender_mpsc_clone = sender_mpsc.clone();
    let name_cloned1 = username.clone();
    let name_cloned2 = username.clone();

    let model_tokenizer_device = state.model_tokenizer_device.lock().unwrap().clone();
    let inference_args = state.inference_args.lock().unwrap().clone();

    // Spawn a task that takes messages from the websocket, prepends the user
    // name, and sends them to all broadcast subscribers.
    let mut recv_join_handle = tokio::spawn(async move {
        while let Some(Ok(Message::Text(msg_in))) = receiver_stream.next().await {
            // This gets sent into send_join_handle and returned immediately.
            // Using to tell user that message is being processed, only sent to original sender.
            if sender_mpsc_clone
                .send("COMING SOON".to_string())
                .await
                .is_err()
            {
                // break on err
                break;
            }

            // Clone msg_in in order to return original message to both clients.
            let msg_in_cloned = msg_in.clone();

            // This allows long running tasks to be sent after logging original message.
            // Process bot message.
            let mut history_cloned = conversation_history.clone();
            let mtd = model_tokenizer_device.clone();
            let name_cloned3 = username.clone();
            let inference_args_cloned = inference_args.clone();
            let work_done = tokio::task::spawn_blocking(move || {
                let bot_msg =
                    create_bot_msg(msg_in, &mut history_cloned, mtd, inference_args_cloned);
                format!("{:?}", bot_msg)
            })
            .await
            .unwrap();

            let forward_message_in = format!("{name_cloned3}: {msg_in_cloned}");

            conversation_history.push(msg_in_cloned);
            conversation_history.push(work_done.clone());
            let _ = broadcast_sender_clone.send(forward_message_in);
            let _ = broadcast_sender_clone.send(format!("Bot: {}", work_done));
        }
    });

    // If any one of the tasks run to completion, we abort the other.
    tokio::select! {
        _ = (&mut send_join_handle) => recv_join_handle.abort(),
        _ = (&mut recv_join_handle) => send_join_handle.abort(),
    }

    // Send "user left" message (similar to "joined" above).
    let msg = format!("{} left.", name_cloned1);
    tracing::debug!("{}", msg);
    let _ = state.broadcast_sender.send(msg);

    // Remove username from map so new clients can take it again.
    state.user_set.lock().unwrap().remove(&name_cloned2);
}
