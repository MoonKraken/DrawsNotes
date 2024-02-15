#[cfg(feature = "ssr")]
use std::str::FromStr;

use serde::{Serialize, Deserialize};
#[cfg(feature = "ssr")]
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Notebook {
    pub id: Option<String>,
    pub name: String,
    pub count: Option<u32>,
}

impl Notebook {
    pub fn new() -> Notebook {
        Notebook {
            id: None,
            name: String::new(),
            count: None,
        }
    }

    pub fn all() -> Notebook {
        Notebook {
            id: None,
            name: "All Notes".to_string(),
            count: None,
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct NotebookNoteCount {
    pub id: String,
    pub count: u32,
}
