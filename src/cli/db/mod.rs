pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::cli::db::models::NewTodoItem;

use self::models::TodoItem;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_todo_item(conn: &mut SqliteConnection, title: &str) -> TodoItem {
    use crate::cli::db::schema::todo_items;

    let new_item = NewTodoItem { title };

    diesel::insert_into(todo_items::table)
        .values(&new_item)
        .returning(TodoItem::as_returning())
        .get_result(conn)
        .expect("Error saving new todo item")
}