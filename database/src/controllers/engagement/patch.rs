use axum::{extract::Path, http::StatusCode, Extension, Json};
use common::database::{engagement::EngagementQuery, msg::Msg};
use sqlx::{Pool, Sqlite};

use crate::types::errors::CustomError;

pub async fn update_engagement(
    Extension(pool): Extension<Pool<Sqlite>>,
    Path(id): Path<i64>,
    Json(updated_engagement): Json<EngagementQuery>,
) -> Result<(StatusCode, Json<Msg>), CustomError> {
    let sql = "UPDATE engagements SET conversation_id=$1 WHERE id=$2";
    let result = sqlx::query(sql)
        .bind(updated_engagement.conversation_id)
        .bind(id)
        .execute(&pool)
        .await;

    let result = format!("Update result: {:?}", result);

    Ok((StatusCode::OK, Json(Msg { msg: result })))
}
