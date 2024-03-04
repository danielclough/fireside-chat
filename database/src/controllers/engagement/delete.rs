use axum::{extract::Path, http::StatusCode, Extension, Json};
use common::database::msg::Msg;
use sqlx::{Pool, Sqlite};

use crate::types::errors::CustomError;
pub async fn delete_engagement_by_id(
    Extension(pool): Extension<Pool<Sqlite>>,
    Path(id): Path<i64>,
) -> Result<(StatusCode, Json<Msg>), CustomError> {
    let delete_result = sqlx::query("DELETE FROM engagements WHERE id=$1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| CustomError::NotFound)?;

    let result = format!("Delete result: {:?}", delete_result);

    Ok((StatusCode::OK, Json(Msg { msg: result })))
}
