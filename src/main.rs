use actix_web::{App, HttpServer};
mod todoitems;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(todoitems::get_all)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}