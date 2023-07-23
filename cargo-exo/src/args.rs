use clap::ArgMatches;

const DEFAULT_FLAGS: &[&str; 2] = &["--quiet", "--color=always"];

const ARG_EXEC: &str = "arg:exec";

pub struct Args {
    pub cmd: String,
}

impl Args {
    pub fn new(args: ArgMatches) -> Self {
        let cmd = args
            .get_one::<String>(ARG_EXEC)
            .cloned()
            .unwrap_or_else(|| "clippy -- -D warnings".to_string());

        Self { cmd }
    }
}
