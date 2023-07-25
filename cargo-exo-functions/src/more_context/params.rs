use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A request for more information that can be used to help solve the error(s)
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct MoreContextParams {
    /// The path to any files that are required to help find a solution to the error(s)
    pub files: Vec<String>,
}
