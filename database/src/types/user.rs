use sqlx::{
    prelude::FromRow,
    types::chrono::{DateTime, Utc},
};

#[derive(Clone, FromRow, Debug)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub active: bool,
    pub created_at: DateTime<Utc>,
}
