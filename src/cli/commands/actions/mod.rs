use crate::cli::commands;
use crate::cli::db;

pub fn create(args: commands::CreateArgs) {
    println!("New list created with name: {}", args.name);
}

pub fn list() {
    println!("List all lists");
}

pub fn add(args: commands::AddArgs) {
    let connection = &mut db::establish_connection();

    let inserted_item = db::create_todo_item( connection, &args.title);

    println!("New todo item created with name: {} and id: {}", inserted_item.title, inserted_item.id);
}

pub fn remove(args: commands::RemoveArgs) {
    println!("Removed from {} with id: {}", args.list, args.id);
}