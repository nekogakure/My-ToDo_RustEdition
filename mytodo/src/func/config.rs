use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub data_file: String,
    pub archive_save_date: u32,
}

pub fn load_or_create_config() -> Config {
    let appdata = dirs_next::config_dir().expect("config directoryの取得に失敗しました");
    let config_dir = PathBuf::from(format!("{}/My-ToDo", appdata.display()));
    let config_file = config_dir.join(".conf");

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).expect("コンフィグファイルの作成に失敗しました");
    }

    if config_file.exists() {
        let mut file =
            File::open(&config_file).expect("コンフィグファイルの読み込みに失敗しました");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("コンフィグファイルの読み込みに失敗しました");

        let mut config: Config = serde_json::from_str(&contents).unwrap_or_else(|_| {
            let mut config: Config =
                serde_json::from_str(&contents).expect("コンフィグのパースに失敗しました");
            config.archive_save_date = 30;
            config
        });

        if config.archive_save_date == 0 {
            config.archive_save_date = 30;
        }

        config
    } else {
        // Default Config
        let default_config = Config {
            data_file: format!("{}/My-ToDo/data/todo.json", appdata.display()),
            archive_save_date: 30,
        };

        let json_string = serde_json::to_string_pretty(&default_config)
            .expect("コンフィグのシリアライズに失敗しました");
        let mut file = File::create(&config_file).expect("コンフィグの作成に失敗しました");
        file.write_all(json_string.as_bytes())
            .expect("コンフィグへの書き込みに失敗しました");

        default_config
    }
}
