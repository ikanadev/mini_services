use actix_web::{get, web, Responder};

use super::utils::get_records_response;
use crate::AppState;

#[get("/records")]
pub async fn top_records(app_data: web::Data<AppState>) -> impl Responder {
    let response = get_records_response(&app_data.db).await;
    web::Json(response)
}
