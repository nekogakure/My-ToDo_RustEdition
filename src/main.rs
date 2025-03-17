use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use serde::{Serialize, Deserialize};


/* DATA */
#[derive(Serialize, Deserialize, Debug)]
struct Config {
    data_file: String,
}

const CONFIG_FILE: &str = "$appdata%/My-ToDo/.conf";


/* main */
fn main() {
    let config_path = Path::new(CONFIG_FILE);
    
    let config: Config = if config_path.exists() {
        let mut file = File::open(CONFIG_FILE).expect("コンフィグファイルの読み込みに失敗しました");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("コンフィグファイルの読み込みに失敗しました");
    
        serde_json::from_str(&contents).expect("コンフィグのパースに失敗しました")
    } else {
        // デフォルト
        let default_config = Config {
            data_file: "%appdata%/My-ToDo/data/todo.json".to_string(),
        };

        let json_string = serde_json::to_string_pretty(&default_config).expect("コンフィグのシリアライズに失敗しました");
        let mut file = File::create(CONFIG_FILE).expect("コンフィグの作成に失敗しました");
        file.write_all(json_string.as_bytes()).expect("コンフィグへの書き込みに失敗しました");

        default_config
    };
    
    println!("CONFIG: {:?}", config);
}