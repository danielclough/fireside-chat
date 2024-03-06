use axum::response::IntoResponse;
use sqlx::{Pool, Sqlite};

// Create TABLE conversations IF NOT EXISTS
pub async fn init(pool: Pool<Sqlite>) -> impl IntoResponse {
    let result = sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS conversations (
id INTEGER PRIMARY KEY NOT NULL,
user_id INTEGER NOT NULL,
name TEXT NOT NULL,
model_params TEXT NOT NULL,
inference_params TEXT NOT NULL,
created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
constraint fk_user_id
    FOREIGN KEY(user_id)
    REFERENCES users(id)
    ON DELETE CASCADE);"#,
    )
    .execute(&pool)
    .await
    .unwrap();
    println!("Create conversations table result: {:?}", result);
}
