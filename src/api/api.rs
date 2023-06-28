use actix_web::web;
use actix_web::{web::{
    Data,
    Json,
}, get, post, put, delete, HttpResponse};
use crate::{
    models::todo::Todo,
    repository::database::Database,
};

#[post("/todos")]
pub async fn create_todo(db: Data<Database>, todo: Json<Todo>) -> HttpResponse {
    let todo: Result<_, _> = db.create_todo(todo.into_inner());
    match todo {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Get all todos
#[get("/todos")]
pub async fn get_todos(db: Data<Database>) -> HttpResponse {
    let todos: Result<_, _> = db.get_all_todos();
    match todos {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

// Get a todo by id
#[get("/todos/{id}")]
pub async fn get_todo_by_id(db: Data<Database>, id: web::Path<String>) -> HttpResponse {
    let todo: Result<_, _> = db.get_todo_by_id(id.into_inner());
    match todo {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[put("/todos/{id}")]
pub async fn update_todo_by_id(db: Data<Database>, id: web::Path<String>, todo: Json<Todo>) -> HttpResponse {
    let todo: Result<_, _> = db.put_todo_by_id(id.into_inner(), todo.into_inner());
    match todo {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[delete("/todos/{id}")]
pub async fn delete_todo_by_id(db: Data<Database>, id: web::Path<String>) -> HttpResponse {
    let todo: Result<_, _> = db.delete_todo_by_id(id.into_inner());
    match todo {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn config(cfn: &mut web::ServiceConfig) {
    cfn.service(web::scope("/api")
        .service(create_todo)
        .service(get_todos)
        .service(get_todos)
        .service(get_todo_by_id)
        .service(update_todo_by_id)
        .service(delete_todo_by_id)
);
}