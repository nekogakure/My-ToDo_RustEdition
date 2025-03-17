use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
struct ToDoData {
    id: String,
    content: String,
    done: bool,
    date: String,
}

pub fn add_todo<P: AsRef<Path>>(data_file: P, content: &str) {
    let mut todos = load_todos(&data_file);
    let new_todo = ToDoData {
        id: Uuid::new_v4().to_string(),
        content: content.to_string(),
        done: false,
        date: chrono::Local::now().to_string(),
    };
    todos.push(new_todo);
    save_todos(&data_file, &todos);
}

pub fn list_todos<P: AsRef<Path>>(data_file: P) {
    let todos = load_todos(&data_file);
    for todo in todos {
        println!("{:?}", todo);
    }
}

pub fn mark_done<P: AsRef<Path>>(data_file: P, id: &str) {
    let mut todos = load_todos(&data_file);
    for todo in &mut todos {
        if todo.id == id {
            todo.done = true;
        }
    }
    save_todos(&data_file, &todos);
}

pub fn delete_todo<P: AsRef<Path>>(data_file: P, id: &str) {
    let mut todos = load_todos(&data_file);
    todos.retain(|todo| todo.id != id);
    save_todos(&data_file, &todos);
}

pub fn show_info<P: AsRef<Path>>(data_file: P) {
    let todos = load_todos(&data_file);
    let total = todos.len();
    let done = todos.iter().filter(|todo| todo.done).count();
    let pending = total - done;
    println!("Total: {}, Done: {}, Pending: {}", total, done, pending);
}

fn load_todos<P: AsRef<Path>>(data_file: P) -> Vec<ToDoData> {
    if !data_file.as_ref().exists() {
        return Vec::new();
    }
    let mut file = File::open(data_file).expect("データファイルの読み込みに失敗しました");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("データファイルの読み込みに失敗しました");
    serde_json::from_str(&contents).expect("データのパースに失敗しました")
}

fn save_todos<P: AsRef<Path>>(data_file: P, todos: &[ToDoData]) {
    let json_string = serde_json::to_string_pretty(todos).expect("データのシリアライズに失敗しました");
    let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(data_file).expect("データファイルの作成に失敗しました");
    file.write_all(json_string.as_bytes()).expect("データファイルへの書き込みに失敗しました");
}
