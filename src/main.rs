use std::io;

use actix_web::{web, App, HttpServer};
use tokio_postgres::NoTls;

use dotenv::dotenv;

use crate::handlers::{get_items, get_todos, status};

mod config;
mod db;
mod handlers;
mod models;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();

    let pool = config.database.create_pool(NoTls).unwrap();

    println!(
        "Starting up the server at http://{}:{}",
        config.server.host, config.server.port
    );

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(status))
            .route("/todos{_:/?}", web::get().to(get_todos))
            .route("/todos{_:/?}", web::post().to(handlers::create_item))
            .route("/todos/{list_id}/items{_:/?}", web::get().to(get_items))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
