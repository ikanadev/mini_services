use crate::{
    api::minesweeper::utils::{board_to_str_arr, cell_map_to_str},
    AppState,
};
use actix_web::{get, post, web, Responder, Scope};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use std::process;

use super::types::{Board, Game, GameLevel};

pub fn minesweeper_routes(path: &str) -> Scope {
    web::scope(path).service(hello).service(save_record)
}

#[get("/records")]
async fn hello() -> impl Responder {
    #[derive(Debug, Serialize, Deserialize)]
    struct DummyBoard {
        board: i32,
    }
    let board = DummyBoard { board: 3 };
    // let serialized = serde_json::to_string(&point).unwrap();
    web::Json(board)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecordInsert {
    game: Game,
    board: Board,
    #[serde(rename = "startedAt")]
    started_at: u64,
    duration: u32,
    name: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct SaveRecordRequest {
    record: RecordInsert,
    #[serde(rename = "gameLevel")]
    game_level: GameLevel,
}
#[post("/record")]
async fn save_record(
    req_data: web::Json<SaveRecordRequest>,
    app_data: web::Data<AppState>,
) -> impl Responder {
    let record_id = Uuid::new_v4();
    println!("{}", record_id);
    let query_result = sqlx::query(
        r#"
        INSERT INTO records
        (id, game_level, started_at, name, duration, game_status, mines_count, flagged, opened, board)
        values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10);
    "#,
    )
    .bind(record_id)
    .bind(req_data.game_level.to_string())
    .bind(req_data.record.started_at as i64)
    .bind(req_data.record.name.to_string())
    .bind(req_data.record.duration as i64)
    .bind(req_data.record.game.status.to_string())
    .bind(req_data.record.game.mines_count as i32)
    .bind(cell_map_to_str(req_data.record.game.flagged_map.clone()))
    .bind(cell_map_to_str(req_data.record.game.opened_map.clone()))
    .bind(board_to_str_arr(req_data.record.board.clone()))
    .execute(&app_data.db)
    .await;
    let query_result = match query_result {
        Ok(res) => res,
        Err(err) => {
            eprintln!("Error saving: {}", err);
            process::exit(1);
        }
    };

    println!("{:?}, {}", req_data, query_result.rows_affected());
    web::Json(req_data)
}
