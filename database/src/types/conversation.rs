use sqlx::{
    prelude::FromRow,
    types::chrono::{DateTime, Utc},
    Decode,
};

use super::engagement::Engagement;

#[derive(Clone, FromRow, Debug)]
pub struct Conversation {
    pub id: i64,
    pub name: String,
    pub user_id: i64,
    pub model_params: String,
    pub inference_params: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Decode)]
pub struct ConversationFull {
    pub id: i64,
    pub name: String,
    pub user_id: i64,
    pub engagements: Vec<Engagement>,
    pub model_params: String,
    pub inference_params: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
