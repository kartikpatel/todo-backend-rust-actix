use std::env;
use actix_cors::Cors;
use actix_web::{
    get,
    http::Method,
    middleware::Logger,
    App, 
    HttpResponse, 
    HttpServer, 
    Responder
};
use serde::Serialize;

#[derive(Serialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool
}

#[get("/")]
async fn index() -> impl Responder {
    let todos:Vec<Todo> = Vec::new();
    HttpResponse::Ok().json(todos)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Get the port number to listen on.
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allowed_methods(vec![
                Method::GET, 
                Method::POST, 
                Method::OPTIONS, 
                Method::PATCH, 
                Method::DELETE
            ]);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .service(index)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}