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
