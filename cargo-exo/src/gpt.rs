use std::{path::Path, time::Duration};

use colored::Colorize;
use dialoguer::Select;
use indicatif::ProgressBar;
use tokio::select;

use crate::{
    api,
    cargo::CargoCommandResult,
    config::Config,
    model::{
        request::{Request, GPT_3_5, GPT_4},
        response::FunctionCall,
    },
};

pub async fn ask_the_robots(
    cmd: &str,
    cmd_result: &CargoCommandResult,
    config: &Config,
    project_root: &Path,
) {
    // Remove everything from output before the first "error: " line
    let output = cmd_result
        .stderr
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
        _ => return,
    };

    let request = Request::new(cmd.to_string(), output, model.to_string());
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
            return;
        }
    };

    match &result.choices[0].message.function_call {
        Some(FunctionCall::UpdateFile(params)) => {
            cargo_exo_functions::update_files::update_files(params, project_root);
        }
        Some(FunctionCall::Explain(params)) => {
            cargo_exo_functions::explain::explain(params);
        }
        Some(FunctionCall::MoreContext(params)) => {
            cargo_exo_functions::more_context::more_context(params, project_root)
        }
        None => {
            println!("🤖 no changes to make!");
        }
    }
}
