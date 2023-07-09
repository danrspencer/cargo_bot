use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    pub error: Error,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Error {
    message: String,
    #[serde(rename = "type")]
    error_type: String,
    param: Option<String>,
    code: Option<String>,
}
