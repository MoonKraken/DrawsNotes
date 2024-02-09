#[cfg(feature = "ssr")]
use std::str::FromStr;

use serde::{Serialize, Deserialize};
#[cfg(feature = "ssr")]
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Notebook {
    pub id: Option<String>,
    pub name: String,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct NotebookDB {
    pub id: Option<Thing>,
    pub name: String,
}

#[cfg(feature = "ssr")]
impl From<NotebookDB> for Notebook {
    fn from(value: NotebookDB) -> Self {
        Notebook {
            id: value.id.map(|id| format!("{}:{}", id.tb, id.id)),
            name: value.name,
        }
    }
}

#[cfg(feature = "ssr")]
impl From<Notebook> for NotebookDB {
    fn from(value: Notebook) -> Self {
        NotebookDB {
            id: value.id.map(|value: String| Thing::from_str(&value).expect("conversion error")),
            name: value.name,
        }
    }
}
