use uuid::Uuid;

use crate::func::load_todo;
use crate::func::save_todo;
use crate::todo_data::ToDoData;
use std::fs;
use std::path::Path;

pub fn add_todo<P: AsRef<Path>>(data_file: P, content: &str) {
    let data_file = data_file.as_ref();
    if let Some(parent) = data_file.parent() {
        if !parent.exists() {
            if let Err(e) = fs::create_dir_all(parent) {
                eprintln!("ディレクトリの作成に失敗しました: {}", e);
                return;
            }
        }
    }
    let mut todos = load_todo::load_todos(data_file);

    let new_todo = ToDoData {
        id: Uuid::new_v5(&Uuid::NAMESPACE_DNS, content.as_bytes()).to_string(),
        content: content.to_string(),
        done: false,
        date: chrono::Local::now().to_string(),
    };

    todos.push(new_todo);
    save_todo::save_todos(data_file, &todos);
}
