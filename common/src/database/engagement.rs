use serde::{Deserialize, Serialize};
use struct_iterable::Iterable;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct EngagementForJson {
    pub id: i64,
    pub conversation_id: i64,
    pub query: String,
    pub response: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EngagementQuery {
    pub id: Option<i64>,
    pub conversation_id: Option<i64>,
    pub keyword: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewEngagement {
    pub conversation_id: i64,
    pub query: String,
    pub response: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Iterable, PartialEq, Eq, Hash)]
pub struct EngagementForJsonVec {
    pub list: Vec<EngagementForJson>,
}
