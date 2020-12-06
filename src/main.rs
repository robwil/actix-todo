mod models;

use crate::models::Status;
use actix_web::{HttpServer, App, web, Responder};
use std::io;

async fn status() -> impl Responder {
    web::HttpResponse::Ok()
        .json(Status{ status: "all good".to_owned() })
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    println!("Starting web server on http://127.0.0.1:8000 ...");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(status))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
