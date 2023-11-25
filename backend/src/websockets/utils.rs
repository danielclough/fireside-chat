use crate::mistral::infer::mistral;
use crate::websockets::types::AppState;

use axum::extract::ws::{Message, WebSocket};
use axum::{
    extract::{ws::WebSocketUpgrade, State},
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};

use std::sync::Arc;

use regex::Regex;

// This function deals with a single websocket connection, i.e., a single
// connected client / user, for which we will spawn two independent tasks (for
// receiving / sending chat messages).
async fn websocket(stream: WebSocket, state: Arc<AppState>) {
    //
    // Mistral
    // Load unique conversation log per user.
    let mut conversation_history: Vec<String> = vec!["Hello! How can I help you!".to_string()];

    // By splitting, we can send and receive at the same time.
    let (mut sender, mut receiver) = stream.split();

    // Username gets set in the receive loop, if it's valid.
    let mut username = String::new();
    // Loop until a text message is found.
    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(name) = message {
            // If username that is sent by client is not taken, fill username string.
            check_username(&state, &mut username, &name);

            // If not empty we want to quit the loop else we want to quit function.
            if !username.is_empty() {
                break;
            } else {
                // Only send our client that username is taken.
                let _ = sender
                    .send(Message::Text(String::from("Username already taken.")))
                    .await;

                return;
            }
        }
    }

    // We subscribe *before* sending the "joined" message, so that we will also
    // display it to our client.
    let mut rx = state.tx.subscribe();

    // Now send the "joined" message to all subscribers.
    let msg = format!("{} joined.", username);
    tracing::debug!("{}", msg);
    let _ = state.tx.send(msg);

    // Spawn the first task that will receive broadcast messages and send text
    // messages over the websocket to our client.
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // In any websocket error, break loop.

            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    // Clone things we want to pass (move) to the receiving task.
    let tx = state.tx.clone();
    let name = username.clone();

    let state_cloned = state.clone();

    // Spawn a task that takes messages from the websocket, prepends the user
    // name, and sends them to all broadcast subscribers.
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text_from_chat))) = receiver.next().await {
            // Add username before message and send on websocket.
            let msg_to_send = format!("{}:  {} ", name, text_from_chat.clone());
            let _ = tx.send(msg_to_send.clone());

            // Create prompt
            let prompt = format!(
                "<s>[INST] Always respond with concise messages with correct grammar. Avoid html tags, garbled content, and words that run into one another. If you don't know the answer to a question say 'I don't know'.[/INST] {} </s>
[INST] {} [/INST]",
                conversation_history
                    .iter()
                    .rev()
                    .take(2)
                    .rev()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(" "),
                text_from_chat.clone()
            );

            tracing::debug!("{}", prompt);

            //
            // Run Mistral
            //
            let regex =
                Regex::new(r"(\[INST\]|\[\/INST\]|\[inst\]|\[\/inst\])").expect("Invalid regex");
            let model_args = state_cloned.model_args.lock().unwrap();
            let bot_response = mistral(prompt, &model_args)
                .unwrap_or("Bot is away at the moment, try again later.".to_string());

            let split_bot_response = regex
                .split(&bot_response)
                .take(1)
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("");

            // Send bot message and add to conversation_history
            let bot_msg_to_send = format!("Bot: {}", split_bot_response.clone());

            conversation_history.push(text_from_chat);
            conversation_history.push(split_bot_response.clone());

            let _ = tx.send(bot_msg_to_send);
        }
    });

    // If any one of the tasks run to completion, we abort the other.
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }

    // Send "user left" message (similar to "joined" above).
    let msg = format!("{} left.", username);
    tracing::debug!("{}", msg);
    let _ = state.tx.send(msg);

    // Remove username from map so new clients can take it again.
    state.user_set.lock().unwrap().remove(&username);
}

fn check_username(state: &AppState, string: &mut String, name: &str) {
    let mut user_set = state.user_set.lock().unwrap();

    if !user_set.contains(name) {
        user_set.insert(name.to_owned());

        string.push_str(name);
    }
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, state))
}
