use crate::models::SuccessMessage;
use crate::models::ErrorMessage;
use crate::db;
use crate::models::CreateTodoList;
use crate::models::Status;
use actix_web::HttpResponse;
use actix_web::{web, Responder};
use deadpool_postgres::{Client, Pool};

pub async fn status() -> impl Responder {
    web::HttpResponse::Ok().json(Status {
        status: "all good".to_owned(),
    })
}

pub async fn get_todos(db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to database");
    let result = db::get_todos(&client).await;
    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        // TODO: extract this error handling logic to some generic matcher
        Err(err) => HttpResponse::InternalServerError().json(ErrorMessage{error: format!("{:#}", err)}),
    }
}

pub async fn get_items(db_pool: web::Data<Pool>, path: web::Path<(i32,)>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to database");
    let result = db::get_items(&client, path.0).await;
    match result {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(err) => HttpResponse::InternalServerError().json(ErrorMessage{error: format!("{:#}", err)}),
    }
}

pub async fn create_todo(
    db_pool: web::Data<Pool>,
    body: web::Json<CreateTodoList>,
) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to database");
    let result = db::create_todo(&client, body.title.clone()).await;
    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => HttpResponse::InternalServerError().json(ErrorMessage{error: format!("{:#}", err)}),
    }
}

pub async fn check_item(db_pool: web::Data<Pool>, path: web::Path<(i32, i32)>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to database");
    let result = db::check_item(&client, path.0, path.1).await;
    match result {
        Ok(_) => HttpResponse::Ok().json(SuccessMessage{success: true}),
        Err(err) => HttpResponse::InternalServerError().json(ErrorMessage{error: format!("{:#}", err)}),
    }
}