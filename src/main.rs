use std::{env, sync::Mutex};
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

impl From<TodoCreate> for Todo {
    fn from(create: TodoCreate) -> Self {
        Self {
            id: 1,
            title: create.title,
            completed: false,
            order: create.order,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct TodoUpdate {
    pub title: Option<String>,
    pub completed: Option<bool>,
    pub order: Option<i32>,
}

#[get("/")]
async fn index(data: web::Data<Mutex<Vec<Todo>>>) -> impl Responder {
    HttpResponse::Ok().json(&mut *data.lock().unwrap())
}

#[post("/")]
async fn create_todo(data: web::Data<Mutex<Vec<Todo>>>, todo_create: web::Json<TodoCreate>) -> impl Responder {
    println!("todo_create: {:?}", &todo_create);
    data.lock().unwrap().push(todo_create.into_inner().into());
    HttpResponse::Ok().json(&mut *data.lock().unwrap())
}

#[get("/{id}")]
async fn get_todo(params: web::Path<IdentifierParams>, data: web::Data<Mutex<Vec<Todo>>>) -> impl Responder {
    println!("id: {:?}", params);
    HttpResponse::Ok().json(&mut *data.lock().unwrap())
}

#[patch("/{id}")]
async fn update_todo(params: web::Path<IdentifierParams>, data: web::Data<Mutex<Vec<Todo>>>, todo_update: web::Json<TodoUpdate>) -> impl Responder {
    println!("id: {:?}", params);
    println!("todo_update: {:?}", &todo_update);
    HttpResponse::Ok().json(&mut *data.lock().unwrap())
}

#[delete("/{id}")]
async fn delete_todo(data: web::Data<Mutex<Vec<Todo>>>, params: web::Path<IdentifierParams>) -> impl Responder {
    println!("id: {:?}", params);
    HttpResponse::Ok().json(&mut *data.lock().unwrap())
}

#[delete("/")]
async fn delete_all(data: web::Data<Mutex<Vec<Todo>>>) -> impl Responder {
    data.lock().unwrap().clear();
    HttpResponse::Ok().json(&mut *data.lock().unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Get the port number to listen on.
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    let todos:Vec<Todo> = Vec::new();
    let data = web::Data::new(Mutex::new(todos));

    HttpServer::new(move || {
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
            .app_data(data.clone())
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