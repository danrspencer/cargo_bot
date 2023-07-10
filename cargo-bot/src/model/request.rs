use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// pub const MODEL: &str = "gpt-4-0613";
pub const MODEL: &str = "gpt-3.5-turbo-0613";

static UPDATE_FILES_ARGS_SCHEMA: Lazy<Value> = Lazy::new(|| {
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
            - Group fixes by the error that needs fixing (e.g. "error: the borrowed expression implements the required traits") and return that error so the user can see it.
            - The error field should contain only the error message and not the file path or line number.
            - When replacing or inserting a line, provide the entire line of code, not just the part that needs to be replaced or inserted.
            - Help blocks from the Rust tools should only be treated as loose suggestions and not the only solution; prefer sensible solutions over suggested ones. 
            - The suggested help from the Rust tool only tells you which part of the line to update, not the entire line. You should update the entire line. See example below.
            - You can update multiple lines at once.
            - Where there is an insert and a delete, prefer a replace.
            - When fixing imports, only remove the unnessesary imports (see example below). If you remove the last import from a line, remove the whole line.
            - Try to fix every error.

            Unnessesary borrow example:
            ```
            18 |         let mut file = File::open(&config_path).expect("Could not open file");
               |                                   ^^^^^^^^^^^^ help: change this to: `config_path`
            ```
            The part of the line that needs to be updated is the `&config_path` part. However, you should update the entire line to `let mut file = File::open(config_path).expect("Could not open file");`.

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
                parameters: UPDATE_FILES_ARGS_SCHEMA.clone(),
            }],
        }
    }
}
