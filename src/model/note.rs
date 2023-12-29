use serde::{Serialize, Deserialize};

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Note {
    pub title: String,
    pub content: String,
}
