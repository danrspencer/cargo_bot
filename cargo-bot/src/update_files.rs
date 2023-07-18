use cargo_bot_params::update_files::{FileUpdate, LineAction, LineUpdate, UpdateFilesArgs};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm};
use serde_json::value::Index;
use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn update_files(args: &UpdateFilesArgs) {
    for file_update in &args.files {
        let path = Path::new(&file_update.file);
        let lines = {
            let file = OpenOptions::new().read(true).open(path).unwrap();
            let reader = BufReader::new(file);
            reader.lines().collect::<Result<_, _>>().unwrap()
        };

        update_file::<UserCli>(file_update, lines);
    }
}

fn update_file<C: Cli>(file_update: &FileUpdate, mut lines: Vec<String>) -> Vec<String> {
    C::display_error(&file_update.cause);

    let confirmed_line_updates = file_update
        .lines
        .iter()
        .filter(|line_update| confirm_update(&file_update.file, line_update, &lines))
        .collect::<Vec<_>>();

    // TODO - We need to keep track of any lines added / removed so we're updating the correct line numbers on subsequent patches
    for line_update in confirmed_line_updates {
        let index = line_update.line_no as usize - 1;

        match line_update.action {
            LineAction::Insert => {
                if let Some(ref content) = line_update.content {
                    lines.insert(index, content.clone());
                }
            }
            LineAction::Replace => {
                if let (Some(ref content), Some(line)) =
                    (&line_update.content, lines.get_mut(index))
                {
                    *line = content.clone();
                }
            }
            LineAction::Delete => {
                lines.remove(index);
            }
        }
    }

    lines

    // for line_update in &file_update.lines {
    //     let updated_lines = show_patch(&file_update.file, line_update, &lines);

    //     if Confirm::with_theme(&ColorfulTheme::default())
    //         .with_prompt("Do you want to apply these changes?")
    //         .default(true)
    //         .interact()
    //         .unwrap()
    //     {
    // //

    // let mut file = OpenOptions::new()
    //     .write(true)
    //     .truncate(true)
    //     .open(path)
    //     .unwrap();

    // for line in &updated_lines {
    //     writeln!(file, "{}", line).unwrap();
    // }

    // lines = updated_lines;
    //     }
    // }
}

fn confirm_update(file: &str, line_update: &LineUpdate, lines: &[String]) -> bool {
    let index = line_update.line_no as usize - 1;
    let indent = " ".repeat(index.to_string().len());

    println!(
        "{}{} {}:{}",
        indent,
        "-->".bright_blue().bold(),
        file,
        line_update.line_no
    );

    match line_update.action {
        LineAction::Insert => {
            if let Some(ref content) = line_update.content {
                println!(
                    "{} {}",
                    format!("{} |", line_update.line_no).bright_blue().bold(),
                    format!("+{}", content).green()
                );
            }
        }
        LineAction::Replace => {
            if let Some(ref content) = line_update.content {
                println!(
                    "{} {}",
                    format!("{} |", line_update.line_no).bright_blue().bold(),
                    format!("-{}", lines[index]).red()
                );
                println!(
                    "{} {}",
                    format!("{} |", line_update.line_no).bright_blue().bold(),
                    format!("+{}", content).green()
                );
            }
        }
        LineAction::Delete => {
            println!(
                "{} {}",
                format!("{} |", line_update.line_no).bright_blue().bold(),
                format!("-{}", lines[index]).red()
            );
        }
    }

    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to apply these changes?")
        .default(true)
        .interact()
        .unwrap()
}

trait Cli {
    fn display_error(error: &str);
}

struct UserCli;

impl Cli for UserCli {
    fn display_error(error: &str) {
        println!();

        let (mut error, message) = error.split_at(error.find(':').unwrap_or(0));
        if error.is_empty() {
            error = "error";
        }
        println!(
            "{}{} {}",
            error.bright_red().bold(),
            ":".bold(),
            message.bold()
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use cargo_bot_params::update_files::UpdateFilesArgs;

    #[test]
    fn it_shows_diff() {
        let update_files = UpdateFilesArgs {
            files: vec![FileUpdate {
                cause: "cannot move out of a shared reference".to_string(),
                file: "lib/crs_controller_allocation/src/distribution/commands.rs".to_string(),
                lines: vec![LineUpdate {
                    line_no: 38,
                    content: Some(
                        "let name = thing.as_ref().map(|t| t.name_unchecked());".to_string(),
                    ),
                    action: LineAction::Replace,
                }],
            }],
        };

        // generate a vector of 40 lines
        let lines = (0..40)
            .into_iter()
            .map(|i| format!("line {}", i.to_string()))
            .collect::<Vec<String>>();

        update_file::<UserCli>(&update_files.files[0], lines);
    }
}
