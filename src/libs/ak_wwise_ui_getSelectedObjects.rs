use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Object {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Result {
    pub objects: Vec<Object>,
}