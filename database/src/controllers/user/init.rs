use axum::response::IntoResponse;
use sqlx::{Pool, Sqlite};

// Create TABLE users IF NOT EXISTS
pub async fn init(pool: Pool<Sqlite>) -> impl IntoResponse {
    let result = sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS users (
id INTEGER PRIMARY KEY NOT NULL,
name VARCHAR(250) UNIQUE NOT NULL,
active BOOLEAN NOT NULL,
created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP);"#,
    )
    .execute(&pool)
    .await
    .unwrap();
    println!("Create users table result: {:?}", result);
}
