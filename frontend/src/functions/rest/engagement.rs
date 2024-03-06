use common::database::engagement::{EngagementForJson, NewEngagement};
use gloo_net::http::Request;

use crate::functions::get_path::get_database_path;

pub async fn post_new_engagement(
    set_args_for_json: NewEngagement,
    database_url: String,
) -> NewEngagement {
    let path = get_database_path("engagement", database_url);

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

pub async fn _patch_existing_engagement(
    set_args_for_json: EngagementForJson,
    database_url: String,
) -> EngagementForJson {
    let path = format!("engagement/id/{}", set_args_for_json.id);
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

pub async fn _get_engagements(database_url: String) -> EngagementForJson {
    let path = get_database_path("engagements", database_url);

    Request::get(&path)
        .send()
        .await
        .expect("Load role list from API")
        .json()
        .await
        .unwrap()
}

pub async fn _get_engagement_by_id(id: i64, database_url: String) -> EngagementForJson {
    let slug = format!("engagement/id/{}", id);
    let path = get_database_path(&slug, database_url);

    Request::get(&path)
        .send()
        .await
        .expect("Load role list from API")
        .json()
        .await
        .unwrap()
}

pub async fn _get_engagements_by_conversation_id(
    id: i64,
    database_url: String,
) -> Vec<EngagementForJson> {
    let slug = format!("engagement/conversation/{}", id);
    let path = get_database_path(&slug, database_url);

    Request::get(&path)
        .send()
        .await
        .expect("Load role list from API")
        .json()
        .await
        .unwrap()
}
