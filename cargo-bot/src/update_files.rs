use cargo_bot_params::update_files::{FileUpdate, LineAction, LineUpdate, UpdateFilesArgs};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm};
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

fn update_file<C: Cli>(file_update: &FileUpdate, lines: Vec<String>) -> Vec<String> {
    C::display_error(&file_update.cause);

    for line_update in &file_update.lines {
        let updated_lines = show_patch(&file_update.file, line_update, &lines);

        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to apply these changes?")
            .default(true)
            .interact()
            .unwrap()
        {
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
        }
    }

    lines
}

// TODO - We need to keep track of any lines added / removed so we're updating the correct line numbers on subsequent patches
fn show_patch(file: &str, line_update: &LineUpdate, lines: &[String]) -> Vec<String> {
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

    // if let Some((line, change)) = change {
    //     let indent_size = vec![original_line_no, updated_line_no, line]
    //         .iter()
    //         .max()
    //         .unwrap()
    //         .to_string()
    //         .len();
    //     let indent = " ".repeat(indent_size);

    //     if original_line_no - last_change_line_no > 1 {
    //         println!(
    //             "{}{} {}:{}",
    //             indent,
    //             "-->".bright_blue().bold(),
    //             file,
    //             original_line_no
    //         );
    //     }
    //     println!("{} {}", format!("{} |", line).bright_blue().bold(), change);
    //     last_change_line_no = original_line_no;
    // }

    // Generate diff
    // let original_contents = lines.join("\n");
    // let updated_contents = updated_lines.join("\n");
    // let changeset = Changeset::new(&original_contents, &updated_contents, "\n");

    // let mut original_line_no = 1;
    // let mut updated_line_no = 1;

    // let mut last_change_line_no = 0;

    // for diff in &changeset.diffs {
    //     let change = match diff {
    //         Difference::Same(ref x) => {
    //             let lines = x.matches('\n').count() + 1;
    //             original_line_no += lines;
    //             updated_line_no += lines;
    //             None
    //         }
    //         Difference::Add(ref x) => {
    //             updated_line_no += 1;
    //             Some((updated_line_no - 1, format!("+{}", x).green()))
    //         }
    //         Difference::Rem(ref x) => {
    //             original_line_no += 1;
    //             Some((original_line_no - 1, format!("-{}", x).red()))
    //         }
    //     };

    //     if let Some((line, change)) = change {
    //         let indent_size = vec![original_line_no, updated_line_no, line]
    //             .iter()
    //             .max()
    //             .unwrap()
    //             .to_string()
    //             .len();
    //         let indent = " ".repeat(indent_size);

    //         if original_line_no - last_change_line_no > 1 {
    //             println!(
    //                 "{}{} {}:{}",
    //                 indent,
    //                 "-->".bright_blue().bold(),
    //                 file,
    //                 original_line_no
    //             );
    //         }
    //         println!("{} {}", format!("{} |", line).bright_blue().bold(), change);
    //         last_change_line_no = original_line_no;
    //     }
    // }

    unimplemented!()
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
    use serde_json::json;

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
