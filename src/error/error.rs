use std::fmt::Display;

use serde::{Serialize, Deserialize};

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum Error {
    Any(String),
    JsonErr(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text: String = match self {
            Self::Any(s) => format!("Any: {}", s),
            Self::JsonErr(s) => format!("JsonErr: {}", s),
        };
        write!(f, "{}", text)
    }
}
