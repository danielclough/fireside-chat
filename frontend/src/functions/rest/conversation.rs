use common::database::{
    conversation::{ConversationForJson, ConversationWithEngagements, NewConversation},
    engagement::EngagementForJsonVec,
};
use gloo_net::http::Request;

use crate::functions::get_path::get_database_path;

pub async fn get_conversations_by_user_id(
    id: i64,
    database_url: String,
) -> Vec<ConversationWithEngagements> {
    let slug = format!("conversations/user_id/{}", id);
    let path = get_database_path(&slug, database_url);

    let default_vec = vec![ConversationWithEngagements {
        id: 0,
        name: "Database Error".to_string(),
        engagements: EngagementForJsonVec { list: vec![] },
        user_id: 0,
        model_params: "Database Error".to_string(),
        inference_params: "Database Error".to_string(),
    }];

    if id == 0 {
        default_vec
    } else {
        let response = Request::get(&path).send().await;
        if response.is_ok() {
            response
                .expect("Load role list from API")
                .json()
                .await
                .unwrap()
        } else {
            default_vec
        }
    }
}

pub async fn post_new_conversation(
    user_id: i64,
    set_args_for_json: NewConversation,
    database_url: String,
) -> ConversationForJson {
    let slug = format!("conversation/{}", user_id);
    let path = get_database_path(&slug, database_url);

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

pub async fn _patch_existing_conversation(
    set_args_for_json: ConversationForJson,
    database_url: String,
) -> ConversationForJson {
    let path = format!("conversation/id/{}", set_args_for_json.id);
    let path = get_database_path(&path, database_url);

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

pub async fn _get_conversations(database_url: String) -> ConversationForJson {
    let path = get_database_path("conversations", database_url);

    Request::get(&path)
        .send()
        .await
        .expect("Load role list from API")
        .json()
        .await
        .unwrap()
}

pub async fn _get_conversation_by_id(id: i64, database_url: String) -> ConversationForJson {
    let slug = format!("conversation/id/{}", id);
    let path = get_database_path(&slug, database_url);

    Request::get(&path)
        .send()
        .await
        .expect("Load role list from API")
        .json()
        .await
        .unwrap()
}
