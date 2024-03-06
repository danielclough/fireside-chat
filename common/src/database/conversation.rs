use serde::{Deserialize, Serialize};
use struct_iterable::Iterable;

use super::engagement::EngagementForJsonVec;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct ConversationForJson {
    pub id: i64,
    pub name: String,
    pub user_id: i64,
    pub model_params: String,
    pub inference_params: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct ConversationWithEngagements {
    pub id: i64,
    pub name: String,
    pub engagements: EngagementForJsonVec,
    pub user_id: i64,
    pub model_params: String,
    pub inference_params: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConversationQuery {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub user_id: Option<i64>,
    pub model_params: Option<String>,
    pub inference_params: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewConversation {
    pub user_id: i64,
    pub name: String,
    pub model_params: String,
    pub inference_params: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Iterable)]
pub struct ConversationForJsonVec {
    pub list: Vec<ConversationForJson>,
}
