mod config;
mod db;
mod handlers;
mod models;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::io;
use tokio_postgres::NoTls;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // Load configuration
    dotenv().ok();
    let config = crate::config::Config::from_env().expect("Problem loading config");

    // Create DB pool
    let pool = config
        .pg
        .create_pool(NoTls)
        .expect("failed to connect to database");

    // Config logger
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Start up web server
    let bind_host_port = format!("{}:{}", config.server.host, config.server.port);
    println!("Starting web server on http://{} ...", bind_host_port);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(pool.clone())
            .route("/", web::get().to(handlers::status))
            .route("/todos{_:/?}", web::get().to(handlers::get_todos))
            .route("/todos/{list_id}/items{_:/?}", web::get().to(handlers::get_items))
    })
    .bind(bind_host_port)?
    .run()
    .await
}
