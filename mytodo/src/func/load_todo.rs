use crate::todo_data::ToDoData;
use std::fs;
use std::path::Path;

pub fn load_todos<P: AsRef<Path>>(data_file: P) -> Vec<ToDoData> {
    if !data_file.as_ref().exists() {
        return Vec::new();
    }

    let contents = fs::read_to_string(data_file).expect("データファイルの読み込みに失敗しました");

    serde_json::from_str(&contents).expect("データのパースに失敗しました")
}
