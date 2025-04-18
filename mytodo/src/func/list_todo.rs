use crate::func::load_todo;
use std::path::Path;

pub fn list_todos<P: AsRef<Path>>(data_file: P) {
    let todos = load_todo::load_todos(data_file);
    println!("========================================");
    println!("Content\t\t| Done\t| Date");
    println!("========================================");

    for todo in todos.iter().rev() {
        println!("{}\t\t| {}\t| {}\t", todo.content, todo.done, todo.date);
        println!("----------------------------------------");
    }
}
