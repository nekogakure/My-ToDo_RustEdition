use crate::todo_data::ToDoData;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub fn save_todos<P: AsRef<Path>>(data_file: P, todos: &[ToDoData]) {
    let json_string =
        serde_json::to_string_pretty(todos).expect("データのシリアライズに失敗しました");

    if !data_file.as_ref().exists() {
        std::fs::create_dir_all(data_file.as_ref().parent().unwrap())
            .expect("ディレクトリの作成に失敗しました");
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(data_file)
        .expect("データファイルの作成に失敗しました");

    file.write_all(json_string.as_bytes())
        .expect("データファイルへの書き込みに失敗しました");
}
