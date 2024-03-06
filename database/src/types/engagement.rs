use sqlx::{
    prelude::FromRow,
    types::chrono::{DateTime, Utc},
};

#[derive(Clone, FromRow, Debug)]
pub struct Engagement {
    pub id: i64,
    pub conversation_id: i64,
    pub query: String,
    pub response: String,
    pub created_at: DateTime<Utc>,
}
