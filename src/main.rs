mod config;
mod models;

use crate::models::Status;
use actix_web::{web, App, HttpServer, Responder};
use dotenv::dotenv;
use std::io;

async fn status() -> impl Responder {
    web::HttpResponse::Ok().json(Status {
        status: "all good".to_owned(),
    })
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let config = crate::config::Config::from_env().expect("Problem loading config");

    let bind_host_port = format!("{}:{}", config.server.host, config.server.port);
    println!("Starting web server on http://{} ...", bind_host_port);
    HttpServer::new(|| App::new().route("/", web::get().to(status)))
        .bind(bind_host_port)?
        .run()
        .await
}
