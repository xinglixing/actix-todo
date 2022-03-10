mod config;
mod models;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use deadpool_postgres::Runtime;
use dotenv::dotenv;
use models::Status;
use tokio_postgres::NoTls;

async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: "Ok".to_string(),
    })
}

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
            .app_data(pool.clone())
            .route("/", web::get().to(status))
    })
    .bind((config.server.host, config.server.port))?
    .run()
    .await
}
