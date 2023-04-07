use std::collections::HashMap;

use actix_web::{get, post, web, Responder};
use serde::{Deserialize, Serialize};

use super::{
    types::MinesweeperResponse,
    utils::{get_and_insert_count, increment_count},
};
use crate::api::ErrorResp;
use crate::AppState;

const PROJECT: &str = "minesweeper";

#[derive(Serialize, Deserialize)]
enum Entity {
    #[serde(rename = "total_attempts")]
    Attempts,
    #[serde(rename = "solved_boards")]
    Solved,
}
impl ToString for Entity {
    fn to_string(&self) -> String {
        match *self {
            Entity::Attempts => "total_attempts".to_string(),
            Entity::Solved => "solved_boards".to_string(),
        }
    }
}

#[get("/minesweeper")]
pub async fn minesweeper_counts(
    app_data: web::Data<AppState>,
) -> Result<impl Responder, ErrorResp> {
    let mut response: MinesweeperResponse = HashMap::new();
    get_and_insert_count(
        PROJECT,
        &Entity::Solved.to_string(),
        &app_data.db,
        &mut response,
    )
    .await?;
    get_and_insert_count(
        PROJECT,
        &Entity::Attempts.to_string(),
        &app_data.db,
        &mut response,
    )
    .await?;
    Ok(web::Json(response))
}

#[derive(Serialize, Deserialize)]
pub struct AddCountReq {
    entity: Entity,
}
#[post("/minesweeper")]
pub async fn add_minesweeper_count(
    req_data: web::Json<AddCountReq>,
    app_data: web::Data<AppState>,
) -> Result<impl Responder, ErrorResp> {
    increment_count(PROJECT, &req_data.entity.to_string(), &app_data.db).await?;
    let mut response: MinesweeperResponse = HashMap::new();
    get_and_insert_count(
        PROJECT,
        &Entity::Solved.to_string(),
        &app_data.db,
        &mut response,
    )
    .await?;
    get_and_insert_count(
        PROJECT,
        &Entity::Attempts.to_string(),
        &app_data.db,
        &mut response,
    )
    .await?;
    Ok(web::Json(response))
}
