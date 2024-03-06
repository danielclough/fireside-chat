use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewUser {
    pub name: String,
    pub active: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserQuery {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub active: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct UserForJson {
    pub id: i64,
    pub name: String,
    pub active: bool,
}

impl UserForJson {
    pub fn error() -> Self {
        Self {
            id: 0,
            name: "Database Error".to_string(),
            active: false,
        }
    }
}
