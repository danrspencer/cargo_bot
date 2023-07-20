struct Args {
    pub cmd: Vec<String>,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            cmd: vec![
                "clippy".to_string(),
                "-q".to_string(),
                "--color=always".to_string(),
                "--".to_string(),
                "-D".to_string(),
                "warnings".to_string(),
            ],
        }
    }
}
