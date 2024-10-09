use std::{fmt::Display, str::FromStr};

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
impl Display for UserForJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
impl FromStr for UserForJson {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(UserForJson {
            id: 0,
            name: s.to_string(),
            active: true,
        })
    }
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
