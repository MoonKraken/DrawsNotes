#[cfg(feature = "server")]
use std::str::FromStr;

use serde::{Serialize, Deserialize};
#[cfg(feature = "server")]
use surrealdb::sql::Id;
#[cfg(feature = "server")]
use surrealdb::sql::Thing;

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Note {
    pub id: Option<String>,
    pub title: String,
    pub content: String,
    pub notebook: String,
}

impl Note {
    pub fn new(notebook: String) -> Note {
        Note {
            id: None,
            title: "New Note".to_string(),
            content: "".to_string(),
            notebook,
        }
    }
}
