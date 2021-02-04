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

struct AppState {
    counter: Mutex<i32>,
    todos: Mutex<Vec<Todo>>,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(&*data.todos.lock().unwrap())
}

#[post("/")]
async fn create_todo(data: web::Data<AppState>, todo_create: web::Json<TodoCreate>) -> impl Responder {
    println!("todo_create: {:?}", &todo_create);

    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    let mut todo: Todo = todo_create.into_inner().into();
    todo.id = *counter;

    let todos = &mut *data.todos.lock().unwrap();
    todos.push(todo);
    
    HttpResponse::Ok().json(todos)
}

#[get("/{id}")]
async fn get_todo(params: web::Path<IdentifierParams>, data: web::Data<AppState>) -> impl Responder {
    println!("id: {:?}", params);

    let todos = &*data.todos.lock().unwrap();
    let todo = todos.iter().find(|x| x.id == params.id);

    HttpResponse::Ok().json(todo)
}

#[patch("/{id}")]
async fn update_todo(params: web::Path<IdentifierParams>, data: web::Data<AppState>, todo_update: web::Json<TodoUpdate>) -> impl Responder {
    println!("id: {:?}", params);
    println!("todo_update: {:?}", &todo_update);

    let todos = &*data.todos.lock().unwrap();
    let todo = todos.iter().find(|x| x.id == params.id);

    HttpResponse::Ok().json(todo)
}

#[delete("/{id}")]
async fn delete_todo(params: web::Path<IdentifierParams>, data: web::Data<AppState>) -> impl Responder {
    println!("id: {:?}", params);

    let todos = &mut *data.todos.lock().unwrap();
    todos.retain(|x| x.id != params.id);

    HttpResponse::Ok().json(todos)
}

#[delete("/")]
async fn delete_all(data: web::Data<AppState>) -> impl Responder {
    let todos = &mut *data.todos.lock().unwrap();
    todos.clear();

    HttpResponse::Ok().json(todos)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Get the port number to listen on.
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    let app_state = web::Data::new(AppState {
        counter: Mutex::new(0),
        todos: Mutex::new(Vec::new())
    });

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
            .app_data(app_state.clone())
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