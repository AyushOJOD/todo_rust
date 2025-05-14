use actix_web::{web, App, HttpServer};

mod models;
mod routes;
mod state;

use routes::todos::*;
use routes::health::health_check;
use state::TodoStore;
use std::collections::HashMap;
use std::sync::Mutex;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running on port 8080");

    let store: web::Data<TodoStore> = web::Data::new(Mutex::new(HashMap::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(store.clone())
            .route("/healthcheck", web::get().to(health_check))
            .route("/todos", web::get().to(get_todos))
            .route("/todos", web::post().to(create_todo))
            .route("/todos/{id}", web::put().to(update_todo))
            .route("/todos/{id}", web::delete().to(delete_todo))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
