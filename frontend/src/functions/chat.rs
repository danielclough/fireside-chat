use common::database::user::UserForJson;
use leptos::*;

pub fn chat_message_state(message: &str, username: String) -> ChatMessageState {
    let join_str = " joined.";
    let joined = format!("{}{}", username, join_str);
    // let else_joined_split_pos = m.char_indices().nth_back(7).unwrap().0;
    // let else_joined_str = m.as_str();
    // let else_joined = &else_joined_str[else_joined_split_pos..];
    let coming_soon = "COMING SOON".to_string();
    let _error = r#"Bot: "\"BOT_ERROR\"""#;
    let is_query = format!("{}: ", username);
    let is_response = "Bot: ";
    if message == joined {
        logging::log!("ChatMessageState::Join");
        ChatMessageState::Join
    } else if &message.len() >= &is_response.len() && &message[0..is_response.len()] == is_response
    {
        logging::log!("ChatMessageState::Bot");
        ChatMessageState::Bot
    } else if message == coming_soon {
        logging::log!("ChatMessageState::Coming");
        ChatMessageState::Coming
    } else if &message.len() >= &is_query.len() && &message[0..is_query.len()] == is_query {
        logging::log!("ChatMessageState::User");
        ChatMessageState::User
    } else {
        logging::log!("ChatMessageState::Error");
        ChatMessageState::Error
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum ChatMessageState {
    Join,
    Coming,
    Error,
    User,
    Bot,
    Waiting,
}

pub fn update_history(
    &history: &WriteSignal<Vec<String>>,
    message: String,
    _user: Signal<UserForJson>,
) {
    let _ = &history.update(|history: &mut Vec<_>| history.push(message));
}

pub fn scroll_down() {
    let el = leptos_dom::document().get_element_by_id("chat-box");
    if let Some(el) = el {
        if el.last_element_child().is_some() {
            el.last_element_child().unwrap().scroll_into_view()
        }
    }
}
