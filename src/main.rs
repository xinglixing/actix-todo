mod config;
mod models;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use models::Status;

async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: "Ok".to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = crate::config::Config::from_env().unwrap();

    println!(
        "Starting server {}:{}",
        config.server.host, config.server.port
    );

    HttpServer::new(|| App::new().route("/", web::get().to(status)))
        .bind((config.server.host, config.server.port))?
        .run()
        .await
}
