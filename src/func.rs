use std::fs::{File, OpenOptions, create_dir_all};
use std::io::{Read, Write};
use std::path::Path;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use uuid::Uuid;
use chrono::{Local, Duration};

#[derive(Serialize, Deserialize, Debug)]
struct ToDoData {
    id: String,
    content: String,
    done: bool,
    date: String,
}

pub fn add_todo<P: AsRef<Path>>(data_file: P, content: &str) {
    let data_file = data_file.as_ref();
    if let Some(parent) = data_file.parent() {
        if !parent.exists() {
            create_dir_all(parent).expect("データファイルのディレクトリ作成に失敗しました");
        }
    }
    let mut todos = load_todos(data_file);
    let new_todo = ToDoData {
        id: Uuid::new_v4().to_string(),
        content: content.to_string(),
        done: false,
        date: chrono::Local::now().to_string(),
    };
    todos.push(new_todo);
    save_todos(data_file, &todos);
}

pub fn list_todos<P: AsRef<Path>>(data_file: P) {
    let todos = load_todos(data_file);
    println!("========================================");
    println!("Content\t\t| Done\t| Date");
    println!("========================================");
    for todo in todos {
        let date = todo.date.split_whitespace().next().unwrap_or("");
        println!("{}\t\t| {}\t| {}", todo.content, todo.done, date);
        println!("----------------------------------------");
    }
}

pub fn mark_done<P: AsRef<Path>>(data_file: P, id: &str) {
    let mut todos = load_todos(&data_file);
    for todo in &mut todos {
        if todo.id == id {
            todo.done = true;
        }
    }
    save_todos(data_file, &todos);
}

pub fn delete_todo<P: AsRef<Path>>(data_file: P, id: &str) {
    let mut todos = load_todos(&data_file);
    todos.retain(|todo| todo.id != id);
    save_todos(&data_file, &todos);
}

pub fn show_info<P: AsRef<Path>>(data_file: P) {
    println!("\t##    ##          #######        #####         ");
    println!("\t##    ##             #           #   ##        ");
    println!("\t###  # # #   #       #     ###   #    #   ###  ");
    println!("\t# #  # #  #  #       #    #  ##  #    ## #  ## ");
    println!("\t# #  # #  # #        #    #   #  #    ## #   # ");
    println!("\t#  ##  #   ##        #    #   #  #    #  #   # ");
    println!("\t#  ##  #   ##        #    #  ##  #   ##  #  ## ");
    println!("\t#      #   #         #     ###   #####    ###  ");
    println!("\t          ##                                   ");
    println!("\t          #                                    ");
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    println!("\n\t\tVersion: {}", VERSION);
    println!("\t\tMade by, nekogakure.");
}

pub fn clean_old_tasks<P: AsRef<Path>>(data_file: P, archive_save_date: u32) {
    let mut todos = load_todos(&data_file);
    let now = Local::now();
    todos.retain(|todo| {
        if todo.done {
            let task_date = chrono::DateTime::parse_from_rfc3339(&todo.date).expect("日付のパースに失敗しました");
            let duration = now.signed_duration_since(task_date);
            duration.num_days() < archive_save_date as i64
        } else {
            true
        }
    });
    save_todos(&data_file, &todos);
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
