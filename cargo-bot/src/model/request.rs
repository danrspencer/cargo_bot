use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

const MODEL: &str = "gpt-4-0613";

static UPDATE_FIlES_ARGS_SCHEMA: Lazy<Value> = Lazy::new(|| {
    let schema = include_str!(concat!(env!("OUT_DIR"), "/update_files_args_schema.json"));
    serde_json::from_str(schema).unwrap()
});

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
            content: r#"You are a Rust tool that uses the output of other Rust tools to automatically fix problems in Rust code. 

            Here are some general guidelines for how you should behave:
            - Help blocks from the Rust tools should only be treated as loose suggestions and not the only solution; prefer sensible solutions over suggested ones. 
            - If you make a change attempt to preserve white space
            - You can update multiple lines at once
            - Where there is an insert and a delete, prefer a replace
            - When fixing imports, only remove the unnessesary imports (see example below)
            - Try to fix every error
            - The error field should contain only the error message and not the file path or line number

            Unused imports example:
            ```
            6 |     fs::{File, OpenOptions},
                         ^^^^
            ```
            The arrows under "File" indicate it is the unused import. You should remove it and leave the rest of the line intact.
            "#.to_string()
        });

        Self {
            model: MODEL.to_string(),
            messages,
            functions: vec![Function {
                name: "update_files".to_string(),
                description: "Update lines in files".to_string(),
                parameters: UPDATE_FIlES_ARGS_SCHEMA,
            }],
        }
    }
}
