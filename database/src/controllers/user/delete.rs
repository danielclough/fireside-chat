use crate::types::errors::CustomError;
use axum::{extract::Path, http::StatusCode, Extension, Json};
use common::database::msg::Msg;
use sqlx::{Pool, Sqlite};

pub async fn delete_user_by_id(
    Extension(pool): Extension<Pool<Sqlite>>,
    Path(id): Path<i64>,
) -> Result<(StatusCode, Json<Msg>), CustomError> {
    let delete_result = sqlx::query("DELETE FROM users WHERE id=$1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| CustomError::NotFound)?;

    let result = format!("Delete result: {:?}", delete_result);

    Ok((StatusCode::OK, Json(Msg { msg: result })))
}

pub async fn delete_user_by_name(
    Extension(pool): Extension<Pool<Sqlite>>,
    Path(id): Path<String>,
) -> Result<(StatusCode, Json<Msg>), CustomError> {
    let delete_result = sqlx::query("DELETE FROM users WHERE name=$1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| CustomError::NotFound)?;

    let result = format!("Delete result: {:?}", delete_result);

    Ok((StatusCode::OK, Json(Msg { msg: result })))
}
