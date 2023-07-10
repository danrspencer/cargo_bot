use std::{fs::File, io::Read, path::Path};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub api_key: String,
}

impl Config {
    pub fn init() -> Self {
        let home_dir = std::env::var("HOME").expect("HOME environment variable not set");

        let config_path = Path::new(&home_dir)
            .join(".cargo")
            .join("cargo-bot-config.toml");

        let mut file = File::open(config_path).expect("Could not open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Could not read file");

        toml::from_str(&contents).unwrap()
    }
}
