use axum::{http::StatusCode, Extension, Json};
use common::database::engagement::NewEngagement;
use sqlx::{Pool, Sqlite};

use crate::types::errors::CustomError;

pub async fn add_engagement(
    Extension(pool): Extension<Pool<Sqlite>>,
    Json(engagement): Json<NewEngagement>,
) -> Result<(StatusCode, Json<NewEngagement>), CustomError> {
    let sql = "INSERT INTO engagements (conversation_id, query, response) VALUES ($1, $2, $3)";

    let _ = sqlx::query(sql)
        .bind(engagement.conversation_id)
        .bind(&engagement.query)
        .bind(&engagement.response)
        .execute(&pool)
        .await
        .map_err(|err| {
            println!("{}", err);
            CustomError::InternalServerError
        })?;

    Ok((StatusCode::CREATED, Json(engagement)))
}
