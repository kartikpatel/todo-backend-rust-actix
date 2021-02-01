use std::env;
use actix_cors::Cors;
use actix_web::{
    delete,
    get,
    patch,
    post,
    http::Method,
    middleware::Logger,
    web,
    App, 
    HttpResponse, 
    HttpServer, 
    Responder
};
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
    pub order: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct IdentifierParams {
    id: i32,
}

#[derive(Deserialize, Debug)]
pub struct TodoCreate {
    pub title: String,
    pub order: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct TodoUpdate {
    pub title: Option<String>,
    pub completed: Option<bool>,
    pub order: Option<i32>,
}

#[get("/")]
async fn index() -> impl Responder {
    let todos:Vec<Todo> = Vec::new();
    HttpResponse::Ok().json(todos)
}

#[post("/")]
async fn create_todo(todo_create: web::Json<TodoCreate>) -> impl Responder {
    println!("todo_create: {:?}", &todo_create);
    let todos:Vec<Todo> = Vec::new();
    HttpResponse::Ok().json(todos)
}

#[get("/{id}")]
async fn get_todo(params: web::Path<IdentifierParams>) -> impl Responder {
    println!("id: {:?}", params);
    let todos:Vec<Todo> = Vec::new();
    HttpResponse::Ok().json(todos)
}

#[patch("/{id}")]
async fn update_todo(params: web::Path<IdentifierParams>, todo_update: web::Json<TodoUpdate>) -> impl Responder {
    println!("id: {:?}", params);
    println!("todo_update: {:?}", &todo_update);
    let todos:Vec<Todo> = Vec::new();
    HttpResponse::Ok().json(todos)
}

#[delete("/{id}")]
async fn delete_todo(params: web::Path<IdentifierParams>) -> impl Responder {
    println!("id: {:?}", params);
    let todos:Vec<Todo> = Vec::new();
    HttpResponse::Ok().json(todos)
}

#[delete("/")]
async fn delete_all() -> impl Responder {
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
            .service(create_todo)
            .service(get_todo)
            .service(update_todo)
            .service(delete_todo)
            .service(delete_all)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}