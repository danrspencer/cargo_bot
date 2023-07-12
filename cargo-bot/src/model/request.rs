use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// pub const MODEL: &str = "gpt-4-0613";
pub const MODEL: &str = "gpt-3.5-turbo-0613";

const SYSTEM_CONTEXT: &str = include_str!("../../../resources/prompts/system.md");

static UPDATE_FILES_ARGS_SCHEMA: Lazy<Value> = Lazy::new(|| {
    let schema = include_str!(concat!(env!("OUT_DIR"), "/update_files_args_schema.json"));
    serde_json::from_str(schema).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub model: String,
    pub temperature: f32,
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

        messages.insert(
            0,
            Message {
                role: Role::System,
                content: SYSTEM_CONTEXT.to_string(),
            },
        );

        Self {
            model: MODEL.to_string(),
            temperature: 0.1,
            messages,
            functions: vec![Function {
                name: stringify!(update_file).to_string(),
                description: "Update lines in files".to_string(),
                parameters: UPDATE_FILES_ARGS_SCHEMA.clone(),
            }],
        }
    }
}
