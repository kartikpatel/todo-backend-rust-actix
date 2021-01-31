use std::env;
use actix_cors::Cors;
use actix_web::{
    middleware::Logger,
    http::Method,
    get,
    App, 
    HttpResponse, 
    HttpServer, 
    Responder
};

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

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}