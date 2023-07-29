use crate::cargo::CargoCommandResult;
use crate::Path;
use cargo_exo_cli::{Cli, UserCli};
use colored::Colorize;
use rustfix::{Filter, Suggestion};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fs;

pub fn get_suggestions(cmd_result: &CargoCommandResult) -> Vec<Suggestion> {
    let messages = cmd_result
        .stdout
        .split('\n')
        .filter_map(|s| serde_json::from_str::<Value>(s).ok())
        .filter_map(|value| value.get("message").cloned());

    messages
        .filter_map(|message| {
            let msg_str = message.to_string();
            // TODO - can we update this to just parse the Value directly?
            rustfix::get_suggestions_from_json(&msg_str, &HashSet::new(), Filter::Everything).ok()
        })
        .flatten()
        .collect::<Vec<_>>()
}

pub fn update_files(suggestions: Vec<Suggestion>, project_root: &Path) {
    let mut files = HashMap::new();
    for suggestion in suggestions {
        let file = suggestion.solutions[0].replacements[0]
            .snippet
            .file_name
            .clone();
        files.entry(file).or_insert_with(Vec::new).push(suggestion);
    }

    for (file, suggestions) in &files {
        let filepath = project_root.join(file);
        let mut source = fs::read_to_string(&filepath).unwrap_or_else(|_| panic!("{:?}", filepath));
        let mut change_counter = 0;

        for suggestion in suggestions.iter().rev() {
            let mut fix = rustfix::CodeFix::new(&source);

            if let Err(e) = fix.apply(suggestion) {
                eprintln!("Failed to apply suggestion to {}: {}", file, e);
            }

            let fixes = fix.finish().unwrap();

            println!();
            println!("{}", suggestion.message.bold());
            if UserCli::confirm_update(file, &source, &fixes) {
                source = fixes;
                change_counter += 1;
            }
        }

        if change_counter > 0 {
            println!("🤖 writing {} changes to {}", change_counter, file);
            fs::write(filepath, source).unwrap();
        }
    }
}
