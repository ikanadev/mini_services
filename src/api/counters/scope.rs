use actix_web::{web, Scope};

pub fn counters_routes(path: &str) -> Scope {
    web::scope(path)
}
