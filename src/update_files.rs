use crate::model::update_files::{FileUpdate, LineAction, LineUpdate, UpdateFilesArgs};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm};
use difference::{Changeset, Difference};
use std::{
    fs::{self, File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    path::Path,
};

pub fn update_files(args: &UpdateFilesArgs) {
    for file_update in &args.files {
        let lines = update_file(file_update);

        // Write the file
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&file_update.file)
            .unwrap();

        for line in lines {
            writeln!(file, "{}", line).unwrap();
        }
    }
    println!()
}

fn update_file(file_update: &FileUpdate) -> Vec<String> {
    let path = &file_update.file;

    let path = Path::new(path);
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);

    let mut lines = reader.lines().collect::<Result<Vec<_>, _>>().unwrap();

    for line_update in &file_update.lines {
        lines = apply_patch(&file_update.file, line_update.clone(), lines);
    }

    lines
}

fn apply_patch(file: &str, line_update: &LineUpdate, lines: Vec<String>) -> Vec<String> {
    let mut updated_lines = lines.clone();

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
                println!();
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

    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to apply these changes?")
        .default(true)
        .interact()
        .unwrap()
    {
        updated_lines
    } else {
        lines
    }
}
