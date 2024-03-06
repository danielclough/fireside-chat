use axum::{extract::Path, http::StatusCode, Extension, Json};
use common::database::conversation::{ConversationForJson, NewConversation};
use sqlx::{Pool, Sqlite};

use crate::types::{conversation::Conversation, errors::CustomError};

pub async fn add_conversation(
    Extension(pool): Extension<Pool<Sqlite>>,
    Path(user_id): Path<i64>,
    Json(conversation): Json<NewConversation>,
) -> Result<(StatusCode, Json<ConversationForJson>), CustomError> {
    let sql = "INSERT INTO conversations (user_id, name, model_params, inference_params ) VALUES ($1, $2, $3, $4); SELECT * FROM conversations WHERE id=last_insert_rowid();";

    let result = sqlx::query_as::<_, Conversation>(sql)
        .bind(user_id)
        .bind(conversation.name)
        .bind(conversation.model_params)
        .bind(conversation.inference_params)
        .fetch_one(&pool)
        .await
        .map_err(|err| {
            println!("{}", err);
            CustomError::InternalServerError
        })?;

    println!("{:?}", result);

    Ok((
        StatusCode::CREATED,
        Json(ConversationForJson {
            id: result.id,
            name: result.name,
            user_id: result.user_id,
            model_params: result.model_params,
            inference_params: result.inference_params,
        }),
    ))
}
