use std::time::Duration;

use indicatif::ProgressBar;
use model::request::Request;
use tokio::select;

use crate::model::response::Response;

mod api;
mod cargo;
mod model;
mod parse_error;
mod update_files;

const SAMPLE: &str = include_str!("../resources/sample.json");

#[tokio::main]
async fn main() {
    println!();
    println!("------- 🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖🤖 -------");
    println!();

    let output = cargo::clippy();
    let errors = parse_error::parse_errors(&output);

    if errors.is_empty() {
        println!("🤖 no errors found!");
        return;
    }

    println!("🤖 found {} errors!", errors.len());

    let request = Request::new(errors);
    let mut request_fut = Box::pin(api::send_request(&request));

    let spinner = ProgressBar::new_spinner();
    spinner.set_message("🤖 thinking ...");
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

    match &result.choices[0].message.function_call {
        Some(model::response::FunctionCall::UpdateFiles(args)) => {
            update_files::update_files(args);
        }
        None => {
            println!("🤖 no changes to make!");
        }
    }

    cargo::fmt();
}
