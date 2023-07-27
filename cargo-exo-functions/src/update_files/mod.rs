use self::cli::{Cli, UserCli};
pub use self::params::*;
use colored::Colorize;
use itertools::Itertools;
use rustfix::Suggestion;
use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::Path,
};

mod cli;
mod params;

pub fn update_files_2(suggestions: Vec<Suggestion>, project_root: &Path) {
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

impl From<Suggestion> for LineUpdate {
    fn from(value: Suggestion) -> Self {
        let snippet = value.snippets[0].clone();
        let replacement = value.solutions[0].replacements[0].replacement.clone();

        Self {
            line_no: snippet.line_range.start.line as i32,
            action: LineAction::Replace,
            content: Some(replacement),
        }
    }
}

pub fn update_files(args: &UpdateFilesParams, project_root: &Path) {
    for file_update in &args.files {
        let path = project_root.join(&file_update.file);
        let lines = {
            let file = OpenOptions::new().read(true).open(&path).unwrap();
            let reader = BufReader::new(file);
            reader.lines().collect::<Result<_, _>>().unwrap()
        };

        let updated_lines = update_lines::<UserCli>(file_update, lines);

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(path)
            .unwrap();

        for line in &updated_lines {
            writeln!(file, "{}", line).unwrap();
        }
    }
}

fn update_lines<C: Cli>(file_update: &FileUpdate, mut lines: Vec<String>) -> Vec<String> {
    C::display_error(&file_update.cause);

    for line_update in file_update
        .lines
        .iter()
        .sorted_by(|a, b| a.line_no.cmp(&b.line_no))
    {
        let mut updated_lines = lines.clone();
        let index = (line_update.line_no - 1) as usize;

        match line_update.action {
            LineAction::Insert => {
                if let Some(ref content) = line_update.content {
                    updated_lines.insert(index, content.clone());
                }
            }
            LineAction::Replace => {
                if let (Some(ref content), Some(line)) =
                    (&line_update.content, updated_lines.get_mut(index))
                {
                    *line = content.clone();
                }
            }
            LineAction::Delete => {
                updated_lines.remove(index);
            }
        }

        if C::confirm_update(
            &file_update.file,
            &lines.join("\n"),
            &updated_lines.join("\n"),
        ) {
            lines = updated_lines;
        }
    }

    lines
}

#[cfg(test)]
mod test {
    use super::*;

    struct FakeCli;

    impl Cli for FakeCli {
        fn display_error(_cause: &str) {}
        fn confirm_update(_file: &str, _line_update: &LineUpdate, _lines: &[String]) -> bool {
            true
        }

        fn confirm_update(
            _filename: &str,
            _original_contents: &str,
            _updated_contents: &str,
        ) -> bool {
            true
        }
    }

    #[test]
    fn it_inserts_lines() {
        let file_update = FileUpdate {
            file: "test.txt".to_string(),
            cause: "Test".to_string(),
            lines: vec![LineUpdate {
                line_no: 1,
                action: LineAction::Insert,
                content: Some("Hello".to_string()),
            }],
        };
        let lines = vec![
            "Line 1".to_string(),
            "Line 2".to_string(),
            "Line 3".to_string(),
        ];
        let expected = vec![
            "Hello".to_string(),
            "Line 1".to_string(),
            "Line 2".to_string(),
            "Line 3".to_string(),
        ];
        let result = update_lines::<FakeCli>(&file_update, lines);
        assert_eq!(result, expected);
    }

    #[test]
    fn it_replaces_lines() {
        let file_update = FileUpdate {
            file: "test.txt".to_string(),
            cause: "Test".to_string(),
            lines: vec![LineUpdate {
                line_no: 2,
                action: LineAction::Replace,
                content: Some("Hello".to_string()),
            }],
        };
        let lines = vec![
            "Line 1".to_string(),
            "Line 2".to_string(),
            "Line 3".to_string(),
        ];
        let expected = vec![
            "Line 1".to_string(),
            "Hello".to_string(),
            "Line 3".to_string(),
        ];
        let result = update_lines::<FakeCli>(&file_update, lines);
        assert_eq!(result, expected);
    }

    #[test]
    fn it_deletes_lines() {
        let file_update = FileUpdate {
            file: "test.txt".to_string(),
            cause: "Test".to_string(),
            lines: vec![LineUpdate {
                line_no: 2,
                action: LineAction::Delete,
                content: None,
            }],
        };
        let lines = vec![
            "Line 1".to_string(),
            "Line 2".to_string(),
            "Line 3".to_string(),
        ];
        let expected = vec!["Line 1".to_string(), "Line 3".to_string()];
        let result = update_lines::<FakeCli>(&file_update, lines);
        assert_eq!(result, expected);
    }
    #[test]
    fn it_keeps_track_of_added_and_removed_lines_to_ensure_correctness() {
        let file_update = FileUpdate {
            file: "test.txt".to_string(),
            cause: "Test".to_string(),
            lines: vec![
                LineUpdate {
                    line_no: 1,
                    action: LineAction::Delete,
                    content: None,
                },
                LineUpdate {
                    line_no: 2,
                    action: LineAction::Replace,
                    content: Some("Hello".to_string()),
                },
                LineUpdate {
                    line_no: 3,
                    action: LineAction::Insert,
                    content: Some("World".to_string()),
                },
            ],
        };
        let lines = vec![
            "Line 1".to_string(),
            "Line 2".to_string(),
            "Line 3".to_string(),
        ];
        let expected = vec![
            "Hello".to_string(),
            "World".to_string(),
            "Line 3".to_string(),
        ];
        let result = update_lines::<FakeCli>(&file_update, lines);
        assert_eq!(result, expected);
    }
}
