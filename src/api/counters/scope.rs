use super::handlers::{add_minesweeper_count, minesweeper_counts};
use actix_web::{web, Scope};

pub fn counters_routes(path: &str) -> Scope {
    web::scope(path)
        .service(minesweeper_counts)
        .service(add_minesweeper_count)
}
