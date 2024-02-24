use common::database::engagement::{EngagementForJson, NewEngagement};
use gloo_net::http::Request;

use crate::functions::get_path::get_database_path;

pub async fn post_new_engagement(set_args_for_json: NewEngagement) -> NewEngagement {
    let path = get_database_path("engagement");

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

pub async fn _patch_existing_engagement(set_args_for_json: EngagementForJson) -> EngagementForJson {
    let path = format!("engagement/id/{}", set_args_for_json.id);
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


pub async fn _get_engagements() -> EngagementForJson {
    let path = get_database_path("engagements");

    Request::get(&path)
        .send()
        .await
        .expect("Load role list from API")
        .json()
        .await
        .unwrap()
}

pub async fn _get_engagement_by_id(id: i64) -> EngagementForJson {
    let slug = format!("engagement/id/{}", id);
    let path = get_database_path(&slug);

    Request::get(&path)
        .send()
        .await
        .expect("Load role list from API")
        .json()
        .await
        .unwrap()
}

pub async fn _get_engagements_by_conversation_id(id: i64) -> Vec<EngagementForJson> {
    let slug = format!("engagement/conversation/{}", id);
    let path = get_database_path(&slug);

    Request::get(&path)
        .send()
        .await
        .expect("Load role list from API")
        .json()
        .await
        .unwrap()
}