use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};

use crate::db;
use crate::models::{CreateTodoRequest, Status};

pub async fn status() -> impl Responder {
    web::HttpResponse::Ok().json(Status {
        status: "OK".to_string(),
    })
}

pub async fn get_todos(db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to database");

    let result = db::get_todos(&client).await;

    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn get_items(db_pool: web::Data<Pool>, path: web::Path<(i32,)>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to database");

    let result = db::get_items(&client, path.0).await;

    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn create_item(
    db_pool: web::Data<Pool>,
    body: web::Json<CreateTodoRequest>,
) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error connecting to database");

    let result = db::create_todo(&client, body.title.clone()).await;

    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}
