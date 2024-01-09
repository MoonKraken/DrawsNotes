use dioxus::prelude::Dep;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
}

impl Note {
    pub fn new() -> Note {
        Note {
            id: "".to_string(),
            title: "New Note".to_string(),
            content: "".to_string(),
        }
    }
}
