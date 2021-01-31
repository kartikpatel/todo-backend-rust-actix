use std::env;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT").unwrap_or(String::from("8080"));
    let addr = format!("127.0.0.1:{}", port); 

    HttpServer::new(|| {
        App::new()
    })
    .bind(addr)?
    .run()
    .await
}