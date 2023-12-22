use diesel::prelude::*;
use super::schema::todo_items;


#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::cli::db::schema::todo_items)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TodoItem {
    pub id: i32,
    pub title: String,
    pub is_complete: i32,
}

#[derive(Insertable)]
#[diesel(table_name = todo_items)]
pub struct NewTodoItem<'a> {
    pub title: &'a str,
}