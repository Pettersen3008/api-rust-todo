use std::fmt::Error;
use chrono::prelude::*;
use std::sync::{Arc, Mutex};

use crate::models::todo::Todo;

pub struct Database {
    pub todos: Arc<Mutex<Vec<Todo>>>,
}

impl Database {
    pub fn new() -> Self {
        let todos = Arc::new(Mutex::new(Vec::new()));
        Database { todos }
    }

    pub fn create_todo(&self, todo: Todo) -> Result<Todo, Error> {
        let mut todos = self.todos.lock().unwrap();
        let id: String = uuid::Uuid::new_v4().to_string();
        let created_at = Utc::now();
        let updated_at = Utc::now();
        let todo = Todo {
            id: Some(id),
            created_at: Some(created_at),
            updated_at: Some(updated_at),
            ..todo
        };
        todos.push(todo.clone());
        Ok(todo)
    }

    pub fn get_all_todos(&self) -> Result<Vec<Todo>, Error> {
        let todos = self.todos.lock().unwrap();
        Ok(todos.clone())
    }

    pub fn get_todo_by_id(
        &self,
        id: String,
    ) -> Result<Todo, Error> {
        let todos = self.todos.lock().unwrap();
        let todo = todos.iter().find(|todo| todo.id == Some(id.clone()));
        match todo {
            Some(todo) => Ok(todo.clone()),
            None => Err(Error),
        }
    }

    pub fn put_todo_by_id(
        &self,
        id: String,
        data: Todo,
    ) -> Result<Todo, Error> {
        let mut todos = self.todos.lock().unwrap();
        let todo = todos.iter_mut().find(|todo| todo.id == Some(id.clone()));
        match todo {
            Some(todo) => {
                todo.title = data.title.clone();
                todo.description = data.description.clone();
                todo.updated_at = Some(Utc::now());
                Ok(todo.clone())
            }
            None => Err(Error),
        }
    }

    pub fn delete_todo_by_id(
        &self,
        id: String,
    ) -> Result<Todo, Error> {
        let mut todos = self.todos.lock().unwrap();
        let todo = todos.iter().position(|todo| todo.id == Some(id.clone()));
        match todo {
            Some(todo) => Ok(todos.remove(todo)),
            None => Err(Error),
        }
    }

}