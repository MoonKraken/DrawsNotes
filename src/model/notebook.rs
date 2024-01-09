use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Notebook {
    pub id: String,
    pub name: String,
}
