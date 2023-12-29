use serde::{Serialize, Deserialize};

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Notebook {
    pub id: String,
    pub name: String,
}
