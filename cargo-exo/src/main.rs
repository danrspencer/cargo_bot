use crate::{args::Args, cargo::CargoCommand};
use cargo::CargoCommandResult;
use chrono::serde;
use clap::{Arg, Command};
use config::Config;
use core::panic;
use dialoguer::{theme::ColorfulTheme, Confirm};
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

    // let cmd = Command::new("cargo")
    //     .bin_name("cargo")
    //     .version(env!("CARGO_PKG_VERSION"))
    //     .author(env!("CARGO_PKG_AUTHORS"))
    //     .disable_help_subcommand(true)
    //     .subcommand_required(true)
    //     .subcommand(
    //         Command::new("exo").arg(
    //             Arg::new("arg:exec")
    //                 .short('x')
    //                 .long("exec")
    //                 .value_name("command")
    //                 .number_of_values(1)
    //                 .help("Cargo command(s) to execute on changes [default: clippy]"),
    //         ),
    //     );
    // let matches = cmd.get_matches();
    // // todo - maybe we want to let people specify multiple commands?
    // let args = Args::new(matches);

    let args = Args {
        cmd: "clippy -- -D warnings".to_string(),
    };
    let cmds = vec![args.cmd];

    for cmd in cmds {
        println!("🤖 {}", cmd);

        let result = CargoCommand::new(&cmd).quiet().color_always().run_silent();

        let output = if result.was_success() {
            continue;
        } else {
            result.stderr
        };

        let json_result = CargoCommand::new(&cmd).message_format_json().run_silent();

        println!("---------");
        let suggestions = json_result
            .stdout
            .split('\n')
            .filter_map(|line| serde_json::from_str::<Value>(line).ok())
            .filter_map(|value| value.get("message").cloned())
            .filter_map(|message| {
                let msg_str = message.to_string();
                rustfix::get_suggestions_from_json(
                    &msg_str,
                    &HashSet::new(),
                    rustfix::Filter::Everything,
                )
                .ok()
            })
            .flatten()
            .collect::<Vec<_>>();

        println!("🤖 {} suggestions", suggestions.len());

        for suggestion in suggestions {
            println!("🤖 {:?}", suggestion);
        }

        println!();
        if !Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Phone a friend? 📞🤖".to_string())
            .default(true)
            .interact()
            .unwrap()
        {
            break;
        }

        let request = Request::new(cmd, output);
        let mut request_fut = Box::pin(api::send_request(&request, config.api_key.clone()));

        let spinner = ProgressBar::new_spinner();
        spinner.set_message(format!("🤖 thinking ... ({})", model::request::MODEL));
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
                cargo_exo_functions::update_files::update_files(args);
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

    // let _ = CargoCommandResult::fmt();
}
