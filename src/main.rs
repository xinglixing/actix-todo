mod config;
mod db;
mod handlers;
mod models;

use crate::handlers::*;
use actix_web::{web, App, HttpServer};
use deadpool_postgres::Runtime;
use dotenv::dotenv;
use tokio_postgres::NoTls;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = crate::config::Config::from_env().unwrap();

    let pool = config.pg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();

    println!(
        "Starting server {}:{}",
        config.server.host, config.server.port
    );

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(status))
            .route("/todos", web::get().to(get_todos))
    })
    .bind((config.server.host, config.server.port))?
    .run()
    .await
}
