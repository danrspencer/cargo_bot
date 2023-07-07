use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

const MODEL: &str = "gpt-4-0613";

#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub model: String,
    pub messages: Vec<Message>,
    pub functions: Vec<Function>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    role: Role,
    content: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Function {
    name: String,
    description: String,
    parameters: Value,
}

impl Request {
    pub fn new(errors: Vec<String>) -> Self {
        let mut messages = errors
            .into_iter()
            .map(|error| Message {
                role: Role::User,
                content: error,
            })
            .collect::<Vec<_>>();

        messages.insert(0, Message {
            role: Role::System,
            content: "You are a Rust tool that uses the output of other Rust tools to automatically fix problems in Rust code. Help blocks from the Rust tools should only be treated as loose suggestions and not the only solution; prefer sensible solutions over suggested ones. If you make a change attempt to preserve white space.".to_string()
        });

        Self {
            model: MODEL.to_string(),
            messages,
            functions: vec![Function {
                name: "update_files".to_string(),
                description: "Update lines in files".to_string(),
                parameters: update_files_params(),
            }],
        }
    }
}

// TODO - Generate this from the struct
fn update_files_params() -> Value {
    json!({
      "type": "object",
      "required": ["files"],
      "properties": {
          "files": {
              "type": "array",
              "items": {
                  "type": "object",
                  "properties": {
                  "file": {
                      "type": "string"
                  },
                  "lines": {
                      "type": "array",
                      "items": {
                      "type": "object",
                      "properties": {
                          "line": {
                          "type": "integer",
                          "minimum": 1
                          },
                          "content": {
                          "type": "string"
                          },
                          "action": {
                              "type": "string",
                              "enum": ["insert", "replace", "delete"]
                          }
                      },
                      "required": ["line", "action"]
                      }
                  }
                  },
                  "required": ["file", "lines"]
              }
          }
      }
    })
}
