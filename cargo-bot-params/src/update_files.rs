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
    /// Replace the contents of the line
    Replace,
    /// Insert a line below the given line number
    /// (e.g. insert at line 3 it will become the new line 3 and the old line 3 will become line 4)
    Insert,
    /// Delete the line
    Delete,
}
