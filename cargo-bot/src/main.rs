use config::Config;
use dialoguer::{theme::ColorfulTheme, Confirm};
use indicatif::ProgressBar;
use model::request::Request;
use std::time::Duration;
use tokio::select;

mod api;
mod cargo;
mod config;
mod model;
mod parse_error;
mod update_files;

// const _SAMPLE: &str = include_str!("../../resources/sample.json");

#[tokio::main]
async fn main() {
    let config = Config::init();

    for cmd in &[cargo::check, cargo::build, cargo::clippy] {
        let output = cmd();
        let errors = parse_error::parse_errors(&output);

        if errors.is_empty() {
            continue;
        }

        println!();
        if !Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!(
                "Found {} errors! Phone a friend? 📞🤖",
                errors.len()
            ))
            .default(true)
            .interact()
            .unwrap()
        {
            continue;
        }

        let request = Request::new(errors);
        let mut request_fut = Box::pin(api::send_request(&request, config.api_key.clone()));

        let spinner = ProgressBar::new_spinner();
        spinner.set_message(format!("🤖 thinking ... ({})", model::request::MODEL));
        let mut interval = tokio::time::interval(Duration::from_millis(50));

        // let result: Response = serde_json::from_str(SAMPLE).unwrap();

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
            Some(model::response::FunctionCall::UpdateFiles(args)) => {
                update_files::update_files(args);
            }
            None => {
                println!("🤖 no changes to make!");
            }
        }
    }

    cargo::fmt();
}
