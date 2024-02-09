#[cfg(feature = "ssr")]
use std::str::FromStr;

use dioxus::prelude::Dep;
use serde::{Serialize, Deserialize};
#[cfg(feature = "ssr")]
use surrealdb::sql::Id;
#[cfg(feature = "ssr")]
use surrealdb::sql::Thing;

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Note {
    pub id: Option<String>,
    pub title: String,
    pub content: String,
    pub notebook: String,
}

#[cfg(feature = "ssr")]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NoteDB {
    pub id: Option<Thing>,
    pub title: String,
    pub content: String,
    pub notebook: Thing,
}

#[cfg(feature = "ssr")]
impl From<NoteDB> for Note {
    fn from(value: NoteDB) -> Self {
        Note {
            id: value.id.map(|id| format!("{}:{}", id.tb, id.id)),
            title: value.title,
            content: value.content,
            notebook: format!("{}:{}", value.notebook.tb, value.notebook.id),
        }
    }
}

#[cfg(feature = "ssr")]
impl From<Note> for NoteDB {
    fn from(value: Note) -> Self {
        NoteDB {
            id: value.id.map(|value: String| Thing::from_str(&value).expect("conversion error")),
            title: value.title,
            content: value.content,
            notebook: Thing::from_str(&value.notebook).expect("conversion error"),
        }
    }
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
