pub use self::params::*;
use cargo_exo_cli::{Cli, UserCli};
use itertools::Itertools;
use rustfix::Suggestion;
use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
    path::Path,
};

mod params;

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

    let rev_sorted_updates = file_update
        .lines
        .iter()
        .sorted_by(|a, b| b.line_no.cmp(&a.line_no));

    for line_update in rev_sorted_updates {
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
