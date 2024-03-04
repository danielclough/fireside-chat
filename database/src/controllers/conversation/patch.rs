use axum::{extract::Path, http::StatusCode, Extension, Json};
use common::database::{conversation::ConversationQuery, msg::Msg};
use sqlx::{Pool, Sqlite};

use crate::types::errors::CustomError;

use super::get::get_conversation_from_db;

pub async fn update_conversation(
    Extension(pool): Extension<Pool<Sqlite>>,
    Path(id): Path<i64>,
    Json(updated_conversation): Json<ConversationQuery>,
) -> Result<(StatusCode, Json<Msg>), CustomError> {
    let conversation = get_conversation_from_db(
        pool.clone(),
        ConversationQuery {
            id: Some(id),
            name: None,
            user_id: None,
            model_params: None,
            inference_params: None,
        },
    )
    .await?;

    let sql = "UPDATE conversations SET user_id=$1 WHERE id=$2";
    let result = sqlx::query(sql)
        .bind(updated_conversation.user_id)
        .bind(conversation.id)
        .execute(&pool)
        .await;

    let result = format!("Update result: {:?}", result);

    Ok((StatusCode::OK, Json(Msg { msg: result })))
}
