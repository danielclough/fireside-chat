use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use common::database::user::{UserForJson, UserQuery};
use sqlx::{Pool, Sqlite};

use crate::types::{
    errors::CustomError,
    user::{self, User},
};

pub async fn get_users(Extension(pool): Extension<Pool<Sqlite>>) -> impl IntoResponse {
    let sql = "SELECT * FROM users ";

    let result = sqlx::query_as::<_, User>(sql)
        .fetch_all(&pool)
        .await
        .unwrap();

    let mut users: Vec<UserForJson> = vec![];

    for user in result.clone() {
        println!(
            "\nid: {}\n name: {}\n active: {}\n created_at: {}",
            user.id, user.name, user.active, user.created_at
        );

        users.push(UserForJson {
            id: user.id,
            name: user.name,
            active: user.active,
        });
    }

    (StatusCode::OK, Json(users))
}

pub async fn get_user_from_db(
    pool: Pool<Sqlite>,
    user: UserQuery,
) -> Result<user::User, CustomError> {
    let result = if user.id.is_some() {
        let sql = "SELECT * FROM users WHERE id=$1";
        sqlx::query_as::<_, User>(sql)
            .bind(user.id)
            .fetch_one(&pool)
            .await
            .map_err(|_| CustomError::NotFound)?
    } else if user.name.is_some() {
        let sql = "SELECT * FROM users WHERE name=$1";
        sqlx::query_as::<_, User>(sql)
            .bind(user.name)
            .fetch_one(&pool)
            .await
            .map_err(|_| CustomError::NotFound)?
    } else {
        let sql = "SELECT * FROM users WHERE active=$1";
        sqlx::query_as::<_, User>(sql)
            .bind(user.active)
            .fetch_one(&pool)
            .await
            .map_err(|_| CustomError::NotFound)?
    };

    Ok(result)
}

pub async fn get_users_from_db(
    pool: Pool<Sqlite>,
    user: UserQuery,
) -> Result<Vec<user::User>, CustomError> {
    let sql = "SELECT * FROM users WHERE active=$1";
    let result = sqlx::query_as::<_, User>(sql)
        .bind(user.active)
        .fetch_all(&pool)
        .await
        .map_err(|_| CustomError::NotFound)?;

    Ok(result)
}

pub async fn get_user_by_id(
    Extension(pool): Extension<Pool<Sqlite>>,
    Path(id): Path<i64>,
) -> Result<Json<UserForJson>, CustomError> {
    let user = UserQuery {
        id: Some(id),
        name: None,
        active: None,
    };
    let result = get_user_from_db(pool, user).await?;

    println!(
        "\nid: {}\n name: {}\n active: {}\n created_at: {}",
        result.id, result.name, result.active, result.created_at
    );

    let user: UserForJson = UserForJson {
        id: result.id,
        name: result.name,
        active: result.active,
    };

    Ok(Json(user))
}

pub async fn get_user_by_name(
    Extension(pool): Extension<Pool<Sqlite>>,
    Path(name): Path<String>,
) -> Result<Json<UserForJson>, CustomError> {
    let user = UserQuery {
        id: None,
        name: Some(name),
        active: None,
    };
    let result = get_user_from_db(pool, user).await;

    let user: UserForJson = if result.is_ok() {
        let existing_user = result.unwrap();
        UserForJson {
            id: existing_user.id,
            name: existing_user.name,
            active: existing_user.active,
        }
    } else {
        UserForJson {
            id: 0,
            name: String::new(),
            active: false,
        }
    };

    Ok(Json(user))
}

pub async fn get_users_by_activity(
    Extension(pool): Extension<Pool<Sqlite>>,
    Path(activity): Path<bool>,
) -> Result<Json<Vec<UserForJson>>, CustomError> {
    let user = UserQuery {
        id: None,
        name: None,
        active: Some(activity),
    };
    let result = get_users_from_db(pool, user).await?;

    let users: Vec<UserForJson> = if result.is_empty() {
        vec![UserForJson {
            id: 0,
            name: "None".to_string(),
            active: true,
        }]
    } else {
        let mut tmp = vec![];
        for u in result {
            println!(
                "\nid: {}\n name: {}\n active: {}\n created_at: {}",
                u.id, u.name, u.active, u.created_at
            );

            tmp.push(UserForJson {
                id: u.id,
                name: u.name,
                active: u.active,
            });
        }
        tmp
    };

    Ok(Json(users))
}
