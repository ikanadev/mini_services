use crate::{
    api::minesweeper::utils::{board_to_str_arr, cell_map_to_str, get_records_response},
    AppState,
};
use actix_web::{get, post, web, Responder, Scope};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use std::process;

use super::types::{Board, Game, GameLevel, Record};

pub fn minesweeper_routes(path: &str) -> Scope {
    web::scope(path).service(top_records).service(save_record)
}

#[derive(Serialize, Deserialize)]
struct RecordsResponse {
    easy: Vec<Record>,
    medium: Vec<Record>,
    expert: Vec<Record>,
}

#[get("/records")]
async fn top_records(app_data: web::Data<AppState>) -> impl Responder {
    let response = get_records_response(&app_data.db).await;
    web::Json(response)
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
    match query_result {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Error saving: {}", err);
            process::exit(1);
        }
    };

    let results = get_records_response(&app_data.db).await;
    web::Json(results)
}
