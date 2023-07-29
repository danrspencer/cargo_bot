use crate::{args::Args, cargo::CargoCommand};

use config::Config;

use serde_json::Value;

use std::path::Path;
use std::path::PathBuf;

use watchexec::{config::InitConfig, handler::PrintDebug};

mod api;
mod args;
mod cargo;
mod config;
mod gpt;
mod model;
mod rustfix;

#[tokio::main]
async fn main() {
    // TODO - Implement watchexec
    // https://github.com/watchexec/watchexec/tree/main/crates/lib
    let mut init = InitConfig::default();
    init.on_error(PrintDebug(std::io::stderr()));

    // let mut runtime = RuntimeConfig::default();
    // runtime.pathset(["watchexec.conf"]);

    // let we = Watchexec::new(init, runtime.clone()).unwrap();
    // let w = we.clone();

    // let c = runtime.clone();

    let config = Config::init();

    let project_root = get_project_root();

    let args = Args::new();

    for cmd in vec![args.cmd] {
        println!("🤖 {}", cmd);

        let result = CargoCommand::new(&cmd).color_always().run(true, true);

        if result.was_success() {
            continue;
        }

        // Get the results in JSON formart for RustFIX - should be fast because we cached it running the last command
        let json_result = CargoCommand::new(&cmd)
            .message_format_json()
            .run(false, false);

        let suggestions = rustfix::get_suggestions(&json_result);

        if !suggestions.is_empty() {
            rustfix::update_files(suggestions, &project_root);
            continue;
        }

        gpt::ask_the_robots(&cmd, &result, &config, &project_root).await;
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
