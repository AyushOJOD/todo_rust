
use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

use crate::models::{Todo, NewTodo};
use crate::state::TodoStore;


pub async fn get_todos(store: web::Data<TodoStore>) -> impl Responder {
    let store = store.lock().unwrap();
    let todos: Vec<&Todo> = store.values().collect();

    HttpResponse::Ok().json(todos)

}


pub async fn create_todo(store: web::Data<TodoStore>, item: web::Json<NewTodo>) -> impl Responder {
    
    let todo = Todo{
        id: Uuid::new_v4(),
        title: item.title.clone(),
        completed: false,
    };

    let mut store = store.lock().unwrap();

    store.insert(todo.id, todo.clone());

    HttpResponse::Ok().json(todo)

}

pub async fn update_todo(
    store: web::Data<TodoStore>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let mut store = store.lock().unwrap();

    if let Some(todo) = store.get_mut(&id) {
        todo.completed = !todo.completed;
        HttpResponse::Ok().json(todo)
    } else {    
        HttpResponse::NotFound().body("Todo not found")
    }
}

pub async fn delete_todo(store: web::Data<TodoStore>, id: web::Path<Uuid>) -> impl Responder{
    let mut store = store.lock().unwrap();

    if let Some(todo) = store.remove(&id) {
        HttpResponse::Ok().json(todo)
    } else {
        HttpResponse::NotFound().body("Todo not found")
    }
}
