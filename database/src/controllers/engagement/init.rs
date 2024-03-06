use axum::response::IntoResponse;
use sqlx::{Pool, Sqlite};

// Create TABLE engagements IF NOT EXISTS
pub async fn init(pool: Pool<Sqlite>) -> impl IntoResponse {
    let result = sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS engagements
(id INTEGER PRIMARY KEY NOT NULL,
conversation_id INTEGER NOT NULL,
query TEXT NOT NULL,
response TEXT NOT NULL,
created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
constraint fk_conversation_id
    FOREIGN KEY(conversation_id)
    REFERENCES conversations(id)
    ON DELETE CASCADE);"#,
    )
    .execute(&pool)
    .await
    .unwrap();
    println!("Create engagements table result: {:?}", result);
}
