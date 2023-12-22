use crate::cli::commands;

pub fn create(args: commands::CreateArgs) {
    println!("New list created with name: {}", args.name);
}

pub fn list() {
    println!("List all lists");
}

pub fn add(args: commands::AddArgs) {
    println!("Added to {} with title: {}", args.list, args.title);
}

pub fn remove(args: commands::RemoveArgs) {
    println!("Removed from {} with id: {}", args.list, args.id);
}