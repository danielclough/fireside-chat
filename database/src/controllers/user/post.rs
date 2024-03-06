use axum::{http::StatusCode, Extension, Json};
use common::database::user::{NewUser, UserForJson};
use sqlx::{Pool, Sqlite};

use crate::types::{errors::CustomError, user::User};

pub async fn add_user(
    Extension(pool): Extension<Pool<Sqlite>>,
    Json(user): Json<NewUser>,
) -> Result<(StatusCode, Json<UserForJson>), CustomError> {
    let sql = "INSERT INTO users (name, active) VALUES ($1, $2); SELECT * FROM users WHERE id=last_insert_rowid();";

    let new_user = sqlx::query_as::<_, User>(sql)
        .bind(&user.name)
        .bind(user.active)
        .fetch_one(&pool)
        .await
        .map_err(|_| CustomError::InternalServerError)?;

    let user_for_json = UserForJson {
        id: new_user.id,
        name: new_user.name,
        active: new_user.active,
    };

    Ok((StatusCode::CREATED, Json(user_for_json)))
}
