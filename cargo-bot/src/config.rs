use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
}

impl Config {
    pub fn init() -> Self {
        let home_dir = std::env::var("HOME").expect("HOME environment variable not set");

        let config_path = Path::new(&home_dir)
            .join(".cargo")
            .join("cargo-bot-config.toml");

        let file = File::open(config_path.clone());

        match file {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents)
                    .expect("Could not read file");
                toml::from_str(&contents).unwrap()
            }
            Err(_) => {
                let api_key = dialoguer::Input::<String>::new()
                    .with_prompt("Enter your API key")
                    .interact_text()
                    .unwrap();

                let config = Config { api_key };

                let mut file = File::create(config_path).unwrap();
                file.write_all(toml::to_string(&config).unwrap().as_bytes())
                    .unwrap();

                config
            }
        }
    }
}
