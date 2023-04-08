use sqlx::{types::Uuid, Pool, Postgres, Row};
use std::collections::HashMap;

use super::types::{Board, BoardCell, CellMap, Game, GameLevel, Record, RecordsResponse, Status};

pub fn cell_map_to_str(cell_map: CellMap) -> String {
    let keys = cell_map.keys().map(|s| s.to_string()).collect::<Vec<_>>();
    keys.join(",")
}

pub fn board_to_str_arr(board: Board) -> Vec<String> {
    board
        .iter()
        .map(|row| {
            row.iter()
                .map(|cell| match cell {
                    BoardCell::Mine => "M".to_string(),
                    BoardCell::Blank => "B".to_string(),
                    BoardCell::Number(num) => num.to_string(),
                })
                .collect::<Vec<String>>()
                .join("")
        })
        .collect::<Vec<String>>()
}

pub async fn get_records_response(pool: &Pool<Postgres>) -> RecordsResponse {
    RecordsResponse {
        easy: get_records(pool, GameLevel::Easy).await,
        medium: get_records(pool, GameLevel::Medium).await,
        expert: get_records(pool, GameLevel::Expert).await,
    }
}

async fn get_records(pool: &Pool<Postgres>, game_level: GameLevel) -> Vec<Record> {
    let mut records: Vec<Record> = Vec::new();

    let sql = r#"
        select r.* from records r
        join (
            select name, min(duration) as duration from records where game_level=$1 group by name
        ) mins on r.name = mins.name and r.duration = mins.duration and r.game_level=$1 order by r.duration limit 10;
        "#;
    let rows = sqlx::query(sql)
        .bind(game_level.to_string())
        .fetch_all(pool)
        .await
        .unwrap();

    for row in rows {
        let id: Uuid = row.get("id");
        let started_at: i64 = row.get("started_at");
        let name: &str = row.get("name");
        let duration: i32 = row.get("duration");
        // game status is always win
        // let game_status: &str = row.get("game_status");
        let mines_count: i32 = row.get("mines_count");
        let flagged: &str = row.get("flagged");
        let opened: &str = row.get("opened");
        let board: Vec<String> = row.get("board");

        let record = Record {
            id: id.to_string(),
            started_at: started_at as u64,
            duration: duration as u32,
            name: name.to_string(),
            board: get_board_from_str(board),
            game: Game {
                status: Status::Win,
                opened_map: get_cell_map_from_str(opened),
                flagged_map: get_cell_map_from_str(flagged),
                mines_count: mines_count as u32,
            },
        };
        records.push(record);
    }
    records
}

fn get_cell_map_from_str(str: &str) -> CellMap {
    let mut cells_map: HashMap<String, bool> = HashMap::new();
    str.split(",").for_each(|cell| {
        cells_map.insert(cell.to_string(), true);
    });
    cells_map
}

fn get_board_from_str(rows: Vec<String>) -> Board {
    rows.iter()
        .map(|row| {
            let cells: Vec<BoardCell> = row
                .chars()
                .map(|char| match char {
                    'M' => BoardCell::Mine,
                    'B' => BoardCell::Blank,
                    other => BoardCell::Number(other.to_string().parse::<u32>().unwrap()),
                })
                .collect();
            cells
        })
        .collect()
}
