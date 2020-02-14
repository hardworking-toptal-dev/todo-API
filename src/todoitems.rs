use actix_web::{HttpResponse, Responder};
use actix_web::get;

#[get("/api/TodoItems")]
async fn get_all() -> impl Responder {
    HttpResponse::Ok()
}