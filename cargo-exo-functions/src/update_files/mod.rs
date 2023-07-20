use self::cli::{Cli, UserCli};
pub use self::params::*;
use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
    path::Path,
};

mod cli;
mod params;

pub fn update_files(args: &UpdateFilesParams) {
    for file_update in &args.files {
        let path = Path::new(&file_update.file);
        let lines = {
            let file = OpenOptions::new().read(true).open(path).unwrap();
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

    let confirmed_line_updates = file_update
        .lines
        .iter()
        .filter(|line_update| C::confirm_update(&file_update.file, line_update, &lines))
        .collect::<Vec<_>>();

    let mut line_offset = -1;
    for line_update in confirmed_line_updates {
        let index = (line_update.line_no + line_offset) as usize;

        match line_update.action {
            LineAction::Insert => {
                if let Some(ref content) = line_update.content {
                    lines.insert(index, content.clone());
                    line_offset += 1;
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
                line_offset -= 1;
            }
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
