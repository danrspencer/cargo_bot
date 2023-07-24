use crate::args::ARG_EXEC;
use crate::model::request::GPT_3_5;
use crate::model::request::GPT_4;
use crate::{args::Args, cargo::CargoCommand};
use cargo_exo_functions::update_files::update_files_2;
use clap::Arg;
use clap::Command;
use colored::Colorize;
use config::Config;
use core::panic;
use dialoguer::Select;
use std::env;
use std::path::Path;
use std::path::PathBuf;

use indicatif::ProgressBar;
use model::request::Request;
use serde_json::Value;
use std::{collections::HashSet, time::Duration};
use tokio::select;

mod api;
mod args;
mod cargo;
mod config;
mod model;

#[tokio::main]
async fn main() {
    let config = Config::init();

    let args: Vec<String> = env::args().collect();
    let was_cargo_run = args[0].contains("target/debug/cargo-exo");

    let project_root = get_project_root();

    let cmd = Command::new("cargo")
        .bin_name("cargo")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .disable_help_subcommand(true)
        .subcommand_required(!was_cargo_run)
        .subcommand(
            Command::new("exo").arg(
                Arg::new(ARG_EXEC)
                    .short('x')
                    .long("exec")
                    .value_name("command")
                    // todo - maybe we want to let people specify multiple commands?
                    .number_of_values(1)
                    .help("Cargo command(s) to execute on changes [default: clippy]"),
            ),
        );

    let matches = cmd.get_matches();

    // If we can't get the subcommand we're doing cargo run so should just use default args
    let args = matches.subcommand_matches("exo").map_or_else(
        || Args {
            cmd: "clippy -- -D warnings".to_string(),
        },
        Args::new,
    );

    let cmds = vec![args.cmd];

    for cmd in cmds {
        println!("🤖 {}", cmd);

        let result = CargoCommand::new(&cmd).color_always().run(true, true);

        let json_result = CargoCommand::new(&cmd)
            .message_format_json()
            .run(false, false);

        let messages = json_result
            .stdout
            .split('\n')
            .filter_map(|s| serde_json::from_str::<Value>(s).ok())
            .filter_map(|value| value.get("message").cloned());

        let suggestions = messages
            .filter_map(|message| {
                let msg_str = message.to_string();
                // TODO - can we update this to just parse the Value directly?
                rustfix::get_suggestions_from_json(
                    &msg_str,
                    &HashSet::new(),
                    rustfix::Filter::Everything,
                )
                .ok()
            })
            .flatten()
            .collect::<Vec<_>>();

        if !suggestions.is_empty() {
            update_files_2(suggestions, &project_root);
            continue;
        }

        let output = if result.was_success() {
            continue;
        } else {
            result.stderr
        };

        // Remove everything from output before the first "error: " line
        let output = output
            .split('\n')
            .skip_while(|line| !line.starts_with("error: "))
            .collect::<Vec<_>>()
            .join("\n");

        println!();
        let model = match Select::new()
            .items(&["GPT 3.5 Turbo", "GPT 4", "Nope"])
            .with_prompt(format!("{}", "Phone a friend? 📞🤖".bold().blue()))
            .default(0)
            .interact()
            .unwrap()
        {
            0 => GPT_3_5,
            1 => GPT_4,
            _ => break,
        };

        let request = Request::new(cmd, output, model.to_string());
        let mut request_fut = Box::pin(api::send_request(&request, config.api_key.clone()));

        let spinner = ProgressBar::new_spinner();
        spinner.set_message(format!("🤖 thinking ... ({})", model));
        let mut interval = tokio::time::interval(Duration::from_millis(50));

        let result = loop {
            select! {
                result = &mut request_fut => {
                    spinner.finish_with_message("🤖 done!");
                    break result;
                },
                _ = interval.tick() => spinner.tick(),
            }
        };

        let result = match result {
            Ok(result) => result,
            Err(e) => {
                println!("🤖 {:?}", e);
                break;
            }
        };

        match &result.choices[0].message.function_call {
            Some(model::response::FunctionCall::UpdateFile(args)) => {
                cargo_exo_functions::update_files::update_files(args, &project_root);
            }
            Some(model::response::FunctionCall::Explain(args)) => {
                cargo_exo_functions::explain::explain(args);
            }
            None => {
                println!("🤖 no changes to make!");
            }
        }

        break;
    }

    let _ = CargoCommand::new("fmt").run(false, false);
}

fn get_project_root() -> PathBuf {
    let output = CargoCommand::new("metadata").run(false, false);

    let metadata: Value =
        serde_json::from_str(&output.stdout).expect("Failed to parse JSON output");

    let workspace_root = metadata["workspace_root"].as_str();

    if let Some(workspace_root) = workspace_root {
        Path::new(workspace_root).to_owned()
    } else {
        // If there is no workspace, use the root of the first package as the project root
        let package_root = metadata["packages"][0]["manifest_path"]
            .as_str()
            .expect("Failed to get package root");

        // The manifest path points to the `Cargo.toml` file, so we need to get the parent directory
        let project_root = Path::new(package_root)
            .parent()
            .expect("Failed to get parent directory");

        project_root.to_owned()
    }
}
