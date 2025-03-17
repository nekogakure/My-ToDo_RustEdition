use std::env;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};

/* DATA */
#[derive(Serialize, Deserialize, Debug)]
struct Config {
    data_file: String,
}

fn main() {
    /* DATA_INPORT */
    let appdata = env::var("APPDATA").expect("APPDATA環境変数の取得に失敗しました");
    let config_dir = PathBuf::from(format!("{}/My-ToDo", appdata));
    let config_file = config_dir.join(".conf");

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).expect("コンフィグファイルの作成に失敗しました");
    }

    let config: Config = if config_file.exists() {
        let mut file = File::open(&config_file).expect("コンフィグファイルの読み込みに失敗しました");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("コンフィグファイルの読み込みに失敗しました");

        serde_json::from_str(&contents).expect("コンフィグのパースに失敗しました")
    } else {
        // デフォルト
        let default_config = Config {
            data_file: format!("{}/My-ToDo/data/todo.json", appdata),
        };

        let json_string = serde_json::to_string_pretty(&default_config).expect("コンフィグのシリアライズに失敗しました");
        let mut file = File::create(&config_file).expect("コンフィグの作成に失敗しました");
        file.write_all(json_string.as_bytes()).expect("コンフィグへの書き込みに失敗しました");

        default_config
    };

    /* MAIN */
    let args: Vec<String> = env::args().collect(); // コマンドライン引数
    let command = &args[1];
    let data_file = Path::new(&config.data_file);
    if args.len() < 2 {
        println!("Usage: todo [add|list|done|del|info] [content]");
        return;
    }
    if args.len() < 3 {
        let content = &args[2]
    }
}