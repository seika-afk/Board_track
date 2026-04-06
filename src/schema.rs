use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GameSchema {
    pub name: String,
    pub creator: String,
    pub plays: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateGameSchema {
    pub name: Option<String>,
    pub creator: Option<String>,
    pub plays: Option<i32>,
}
