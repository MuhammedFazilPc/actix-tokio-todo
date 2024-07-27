use std::sync::{Arc, Mutex};

// src/routes/handhandlers.rs
use crate::models::models::{add_item, delete_item, get_all_items, update_item};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio_postgres::Client;
#[derive(Serialize, Deserialize, Debug)]
pub struct TodoItem {
    pub title: String,
    pub description: String,
}
#[post("/add")]
pub async fn create_todo(
    db_pool: web::Data<Arc<Mutex<Client>>>,
    item: web::Json<TodoItem>,
) -> impl Responder {
    let client = db_pool.lock().unwrap();
    println!("{:?}", &item);
    match add_item(&client, &item.title, &item.description).await {
        Ok(todo_item) => HttpResponse::Ok().json(todo_item),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
    // "hi there"
}
#[get("/todos")]
pub async fn get_all(db_pool: web::Data<Arc<Mutex<Client>>>) -> impl Responder {
    let client = db_pool.lock().unwrap();
    println!("Client acquired for get_all_items ");
    match get_all_items(&client).await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[put("/todos/{id}")]
pub async fn update_todo(
    db_pool: web::Data<Arc<Mutex<Client>>>,
    id: web::Path<i32>,
) -> impl Responder {
    let client = db_pool.lock().unwrap();
    let response=json!({
        "checked":true
    });
    match update_item(&client, *id).await {
        Ok(_) => HttpResponse::Ok().json(response),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
#[delete["/delete/{id}"]]
pub async fn delete(
    db_pool: web::Data<Arc<Mutex<Client>>>,
    path: web::Path<i32>,
) -> impl Responder {
    let id: i32 = path.into_inner();
    let client = db_pool.lock().unwrap();
    match delete_item(&client, id).await {
        Ok(_) => HttpResponse::Ok().body(format!("deleted item id {}", id)),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
#[get("/")]
async fn start() -> impl Responder {
    let response = json!({
        "status": "Up"
    });

    HttpResponse::Ok().json(response)
}
