use clap::{Arg, Command};
use std::env;

pub const ARG_EXEC: &str = "arg:exec";

pub struct Args {
    pub cmd: String,
}

impl Args {
    pub fn new() -> Self {
        let args: Vec<String> = env::args().collect();
        let was_cargo_run = args[0].contains("target/debug/cargo-exo");

        let cmd = Command::new("cargo")
        .bin_name("cargo")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .disable_help_subcommand(true)
        .subcommand_required(!was_cargo_run)
        .subcommand(
            Command::new("exo")
                .arg(
                    Arg::new(ARG_EXEC)
                        .short('x')
                        .long("exec")
                        .value_name("command")
                        // todo - maybe we want to let people specify multiple commands?
                        .number_of_values(1)
                        .help("Cargo command(s) to execute on changes [default: clippy]"),
                )
                .arg(
                    Arg::new("watch")
                        .short('w')
                        .long("watch")
                        .value_name("watch")
                        .help("Watch files for changes, pauses while interacting with suggestions"),
                ),
        );

        let matches = cmd.get_matches();

        // If we can't get the subcommand we're doing cargo run so should just use default args
        let cmd = matches.subcommand_matches("exo").map_or_else(
            || "clippy -- -D warnings".to_string(),
            |args| {
                args.get_one::<String>(ARG_EXEC)
                    .cloned()
                    .unwrap_or_else(|| "clippy -- -D warnings".to_string())
            },
        );

        Self { cmd }
    }
}
