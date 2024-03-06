use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RoleListEntry {
    pub role: String,
    pub prompt: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RoleList {
    pub human: Vec<RoleListEntry>,
    pub computer: Vec<RoleListEntry>,
}
