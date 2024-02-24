use common::database::conversation::{ConversationForJson, ConversationWithEngagements, NewConversation};
use gloo_net::http::Request;

use crate::functions::get_path::get_database_path;

pub async fn get_conversations_by_user_id(id: i64) -> Vec<ConversationWithEngagements> {
    let slug = format!("conversations/user_id/{}", id);
    let path = get_database_path(&slug);

    println!("{}",id);

    Request::get(&path)
        .send()
        .await
        .expect("Load role list from API")
        .json()
        .await
        .unwrap()
}

pub async fn post_new_conversation(user_id: i64, set_args_for_json: NewConversation) -> ConversationForJson {

    let slug = format!("conversation/{}", user_id);
    let path = get_database_path(&slug);

    Request::post(&path)
        .header("Content-Type", "application/json")
        .json(&set_args_for_json)
        .unwrap()
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

pub async fn _patch_existing_conversation(set_args_for_json: ConversationForJson) -> ConversationForJson {
    let path = format!("conversation/id/{}", set_args_for_json.id);
    let path = get_database_path(&path);

    Request::patch(&path)
        .header("Content-Type", "application/json")
        .json(&set_args_for_json)
        .unwrap()
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}


pub async fn _get_conversations() -> ConversationForJson {
    let path = get_database_path("conversations");

    Request::get(&path)
        .send()
        .await
        .expect("Load role list from API")
        .json()
        .await
        .unwrap()
}

pub async fn _get_conversation_by_id(id: i64) -> ConversationForJson {
    let slug = format!("conversation/id/{}", id);
    let path = get_database_path(&slug);

    Request::get(&path)
        .send()
        .await
        .expect("Load role list from API")
        .json()
        .await
        .unwrap()
}