use cargo_bot_params::update_files::{FileUpdate, LineAction, LineUpdate, UpdateFilesArgs};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm};
use difference::{Changeset, Difference};
use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
    path::Path,
};

pub fn update_files(args: &UpdateFilesArgs) {
    for file_update in &args.files {
        update_file(file_update);
    }
}

fn update_file(file_update: &FileUpdate) -> Vec<String> {
    let path = Path::new(&file_update.file);
    let mut lines: Vec<String> = {
        let file = OpenOptions::new().read(true).open(path).unwrap();
        let reader = BufReader::new(file);
        reader.lines().collect::<Result<_, _>>().unwrap()
    };

    println!();

    let (mut error, message) = file_update
        .error
        .split_at(file_update.error.find(':').unwrap_or(0));
    if error.is_empty() {
        error = "error";
    }
    println!(
        "{}{} {}",
        error.bright_red().bold(),
        ":".bold(),
        message.bold()
    );

    for line_update in &file_update.lines {
        let updated_lines = apply_patch(&file_update.file, line_update, &lines);

        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to apply these changes?")
            .default(true)
            .interact()
            .unwrap()
        {
            let mut file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(path)
                .unwrap();

            for line in &updated_lines {
                writeln!(file, "{}", line).unwrap();
            }

            lines = updated_lines;
        }
    }

    lines
}

// TODO - We need to keep track of any lines added / removed so we're updating the correct line numbers on subsequent patches
fn apply_patch(file: &str, line_update: &LineUpdate, lines: &[String]) -> Vec<String> {
    let mut updated_lines = (*lines).to_vec();

    match line_update.action {
        LineAction::Insert => {
            if let Some(ref content) = line_update.content {
                let index = line_update.line as usize - 1;
                updated_lines.insert(index, content.clone());
            }
        }
        LineAction::Replace => {
            if let Some(ref content) = line_update.content {
                let index = line_update.line as usize - 1;
                // Replace
                *updated_lines
                    .get_mut(index)
                    .ok_or("Line number out of range")
                    .unwrap() = content.clone();
            }
        }
        LineAction::Delete => {
            let index = line_update.line as usize - 1;

            updated_lines.remove(index);
        }
    }

    // Generate diff
    let original_contents = lines.join("\n");
    let updated_contents = updated_lines.join("\n");
    let changeset = Changeset::new(&original_contents, &updated_contents, "\n");

    let mut original_line_no = 1;
    let mut updated_line_no = 1;

    let mut last_change_line_no = 0;

    for diff in &changeset.diffs {
        let change = match diff {
            Difference::Same(ref x) => {
                let lines = x.matches('\n').count() + 1;
                original_line_no += lines;
                updated_line_no += lines;
                None
            }
            Difference::Add(ref x) => {
                updated_line_no += 1;
                Some((updated_line_no - 1, format!("+{}", x).green()))
            }
            Difference::Rem(ref x) => {
                original_line_no += 1;
                Some((original_line_no - 1, format!("-{}", x).red()))
            }
        };

        if let Some((line, change)) = change {
            let indent_size = vec![original_line_no, updated_line_no, line]
                .iter()
                .max()
                .unwrap()
                .to_string()
                .len();
            let indent = " ".repeat(indent_size);

            if original_line_no - last_change_line_no > 1 {
                println!(
                    "{}{} {}:{}",
                    indent,
                    "-->".bright_blue().bold(),
                    file,
                    original_line_no
                );
            }
            println!("{} {}", format!("{} |", line).bright_blue().bold(), change);
            last_change_line_no = original_line_no;
        }
    }

    updated_lines
}
