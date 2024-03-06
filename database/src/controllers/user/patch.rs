use axum::{extract::Path, http::StatusCode, Extension, Json};
use sqlx::{Pool, Sqlite};

use common::database::user::{UserForJson, UserQuery};

use crate::types::errors::CustomError;

use super::get::get_user_from_db;

/// Updates user indicated by ID at Path to values found in JSON.
pub async fn update_user(
    Extension(pool): Extension<Pool<Sqlite>>,
    Path(id): Path<i64>,
    Json(updated_user): Json<UserQuery>,
) -> Result<(StatusCode, Json<UserForJson>), CustomError> {
    let user = get_user_from_db(
        pool.clone(),
        UserQuery {
            id: Some(id),
            name: None,
            active: None,
        },
    )
    .await?;

    let sql = "UPDATE users SET id=$1, name=$2, active=$3 WHERE id=$4";
    let result = sqlx::query(sql)
        .bind(updated_user.id)
        .bind(&updated_user.name)
        .bind(updated_user.active)
        .bind(user.id)
        .execute(&pool)
        .await;

    println!("Update result: {:?}", result);

    Ok((
        StatusCode::OK,
        Json(UserForJson {
            id: updated_user.id.unwrap_or(user.id),
            name: updated_user.name.unwrap_or(user.name),
            active: updated_user.active.unwrap_or(user.active),
        }),
    ))
}
