use std::{env, process};

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenvy;
use sqlx::postgres::{PgPool, PgPoolOptions};

mod api;
use api::minesweeper_routes;

pub struct AppState {
    db: PgPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let db_url = match env::var("DATABASE_URL") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("DATABASE_URL env not set");
            process::exit(1);
        }
    };

    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url.as_str())
        .await
    {
        Ok(pool) => pool,
        Err(err) => {
            eprintln!("DB connection error: {}", err);
            process::exit(1);
        }
    };

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .service(
                // minesweeper_scope
                minesweeper_routes("/minesweeper"),
            )
    })
    .bind(("0.0.0.0", 4000))?
    .run()
    .await
}
