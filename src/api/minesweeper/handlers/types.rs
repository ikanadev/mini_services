use std::collections::HashMap;

use serde::{de, Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct Record {
    pub id: String,
    pub game: Game,
    pub board: Board,
    #[serde(rename = "startedAt")]
    pub started_at: u64,
    pub duration: u32,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct RecordsResponse {
    pub easy: Vec<Record>,
    pub medium: Vec<Record>,
    pub expert: Vec<Record>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub status: Status,
    #[serde(rename = "openedMap")]
    pub opened_map: CellMap,
    #[serde(rename = "flaggedMap")]
    pub flagged_map: CellMap,
    #[serde(rename = "minesCount")]
    pub mines_count: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum GameLevel {
    #[serde(rename = "easy")]
    Easy,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "expert")]
    Expert,
}
impl ToString for GameLevel {
    fn to_string(&self) -> String {
        match self {
            GameLevel::Easy => String::from("easy"),
            GameLevel::Medium => String::from("medium"),
            GameLevel::Expert => String::from("expert"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BoardCell {
    Blank,
    Mine,
    Number(u32),
}
impl Serialize for BoardCell {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            BoardCell::Blank => serializer.serialize_str("B"),
            BoardCell::Mine => serializer.serialize_str("M"),
            BoardCell::Number(num) => serializer.serialize_u32(*num),
        }
    }
}
impl<'de> Deserialize<'de> for BoardCell {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Deserialize::deserialize(deserializer)?;
        match value {
            Value::String(s) => match s.as_str() {
                "B" => Ok(BoardCell::Blank),
                "M" => Ok(BoardCell::Mine),
                _ => Err(de::Error::custom(format!("invalid str value: {}", s))),
            },
            Value::Number(n) => match n.as_u64() {
                Some(num) => Ok(BoardCell::Number(num as u32)),
                None => Err(de::Error::custom(format!("invalid number val: {}", n))),
            },
            _ => Err(de::Error::custom(format!("invalid value"))),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Status {
    #[serde(rename = "win")]
    Win,
    #[serde(rename = "lose")]
    Lose,
    #[serde(rename = "started")]
    Started,
    #[serde(rename = "ready")]
    Ready,
}
impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::Win => String::from("win"),
            Status::Lose => String::from("lose"),
            Status::Started => String::from("started"),
            Status::Ready => String::from("ready"),
        }
    }
}

pub type CellMap = HashMap<String, bool>;
pub type Board = Vec<Vec<BoardCell>>;
