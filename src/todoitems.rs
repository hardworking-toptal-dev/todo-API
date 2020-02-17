use actix_web::{HttpResponse, Result};
use actix_web::get;
use serde::{Deserialize, Serialize};

#[get("/api/TodoItems")]
async fn get_all() -> Result<HttpResponse> {
    let mut todo_items: Vec<TodoItem> = Vec::new();
    todo_items.push(TodoItem::new("Learn Rust".to_string()));
    todo_items.push(TodoItem::new("Learn Actix web".to_string()));
    todo_items.push(TodoItem::new("Learn Ruby on Rails".to_string()));
    todo_items.push(TodoItem::new("Learn Python".to_string()));

    Ok(HttpResponse::Ok().json(todo_items))
}

#[derive(Serialize, Deserialize)]
struct TodoItem {
    description: String,
    is_complete: bool
}

impl TodoItem {
    pub fn new(description: String) -> Self {
        Self {
            description,
            is_complete: false,
        }
    }
}