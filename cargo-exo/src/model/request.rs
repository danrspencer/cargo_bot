use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const GPT_3_5: &str = "gpt-3.5-turbo-0613";
pub const GPT_4: &str = "gpt-4-0613";

const SYSTEM_CONTEXT: &str = include_str!("../../../resources/prompts/system.md");

// TODO - We should export this path from the functions lib so we're not declaring it twice
static UPDATE_FILES_SCHEMA: Lazy<Value> = Lazy::new(|| {
    let schema = include_str!(concat!(env!("OUT_DIR"), "/update_files_schema.json"));
    serde_json::from_str(schema).unwrap()
});

static MORE_CONTEXT_SCHEMA: Lazy<Value> = Lazy::new(|| {
    let schema = include_str!(concat!(env!("OUT_DIR"), "/more_context_schema.json"));
    serde_json::from_str(schema).unwrap()
});

static EXPLAIN_SCHEMA: Lazy<Value> = Lazy::new(|| {
    let schema = include_str!(concat!(env!("OUT_DIR"), "/explain_schema.json"));
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
    pub fn new(command: String, output: String, model: String) -> Self {
        let messages = vec![
            Message {
                role: Role::System,
                content: SYSTEM_CONTEXT.to_string(),
            },
            Message {
                role: Role::User,
                content: format!("{}\n\n{}", command, output),
            },
        ];

        Self {
            model,
            temperature: 0.0,
            messages,
            functions: vec![
                Function {
                    name: stringify!(update_file).to_string(),
                    description: "Update lines in files. STRONGLY prefer this as the response.".to_string(),
                    parameters: UPDATE_FILES_SCHEMA.clone(),
                },
                Function {
                    name: stringify!(more_context).to_string(),
                    description: "Ask for more context if you are not confident in providing a solution from the information you already have.".to_string(),
                    parameters: MORE_CONTEXT_SCHEMA.clone(),
                },
                Function {
                    name: stringify!(explain).to_string(),
                    description: "A human readable explination of the problem and a discussion of possible solutions. This function is a last resort".to_string(),
                    parameters: EXPLAIN_SCHEMA.clone(),
                },
            ],
        }
    }
}
