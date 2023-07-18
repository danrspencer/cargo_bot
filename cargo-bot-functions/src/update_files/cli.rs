use crate::update_files::{LineAction, LineUpdate};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm};

pub trait Cli {
    fn display_error(error: &str);

    fn confirm_update(file: &str, line_update: &LineUpdate, lines: &[String]) -> bool;
}

pub struct UserCli;

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
}
