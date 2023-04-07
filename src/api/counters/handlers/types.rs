use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub type MinesweeperResponse = HashMap<String, i32>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Counter {
    pub id: String,
    pub project: String,
    pub entity: String,
    pub count: i32,
}
