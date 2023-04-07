use actix_web::{web, Scope};

use super::handlers::{save_record_handler, top_records_handler};

pub fn minesweeper_routes(path: &str) -> Scope {
    web::scope(path)
        .service(top_records_handler)
        .service(save_record_handler)
}
