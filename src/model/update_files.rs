use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateFilesArgs {
    pub files: Vec<FileUpdate>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileUpdate {
    pub file: String,
    pub lines: Vec<LineUpdate>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LineUpdate {
    pub line: i32,
    pub content: Option<String>,
    pub action: LineAction,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LineAction {
    Replace,
    Insert,
    Delete,
}
