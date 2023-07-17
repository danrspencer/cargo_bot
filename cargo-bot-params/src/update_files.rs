use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct UpdateFilesArgs {
    pub files: Vec<FileUpdate>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct FileUpdate {
    /// The original error message that was returned from the compiler.
    pub cause: String,
    /// The file to be updated.
    pub file: String,
    /// The lines to be updated.
    pub lines: Vec<LineUpdate>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct LineUpdate {
    /// The line number to be updated
    pub line_no: i32,
    /// The content of the line to be updated
    pub content: Option<String>,
    /// The action to be taken on the line
    pub action: LineAction,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum LineAction {
    Replace,
    Insert,
    Delete,
}
