use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct UpdateFilesArgs {
    pub files: Vec<FileUpdate>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct FileUpdate {
    pub file: String,
    pub error: String,
    pub lines: Vec<LineUpdate>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct LineUpdate {
    pub line: i32,
    pub content: Option<String>,
    pub action: LineAction,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum LineAction {
    Replace,
    Insert,
    Delete,
}
