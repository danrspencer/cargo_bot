use serde::{Deserialize, Deserializer, Serialize};

use super::update_files::UpdateFilesArgs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    pub index: i32,
    pub message: Message,
    pub finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: Option<String>,
    pub function_call: Option<FunctionCall>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "name", content = "arguments", rename_all = "snake_case")]
pub enum FunctionCall {
    #[serde(deserialize_with = "deserialize_update_files_args")]
    UpdateFiles(UpdateFilesArgs),
}

fn deserialize_update_files_args<'de, D>(deserializer: D) -> Result<UpdateFilesArgs, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    serde_json::from_str(&s).map_err(serde::de::Error::custom)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_parses_a_response() {
        let response = r#"{
            "id": "chatcmpl-7Zht9Xvqn1j96D9NisM76cTJkXG1K",
            "object": "chat.completion",
            "created": 1688744543,
            "model": "gpt-4-0613",
            "choices": [
              {
                "index": 0,
                "message": {
                  "role": "assistant",
                  "content": null,
                  "function_call": {
                    "name": "update_files",
                    "arguments": "{\n  \"files\": [\n    {\n      \"file\": \"src/api.rs\",\n      \"lines\": [\n        {\n          \"action\": \"replace\",\n          \"content\": \"\",\n          \"line\": 2\n        }\n      ]\n    },\n    {\n      \"file\": \"src/update_files.rs\",\n      \"lines\": [\n        {\n          \"action\": \"replace\",\n          \"content\": \"// pub fn update_files(args: UpdateFilesArgs) {\",\n          \"line\": 3\n        }\n      ]\n    },\n    {\n      \"file\": \"src/model/request.rs\",\n      \"lines\": [\n        {\n          \"action\": \"replace\",\n          \"content\": \"content: error,\",\n          \"line\": 40\n        }\n      ]\n    }\n  ]\n}"
                  }
                },
                "finish_reason": "function_call"
              }
            ],
            "usage": {
              "prompt_tokens": 387,
              "completion_tokens": 163,
              "total_tokens": 550
            }
          }"#;

        serde_json::from_str::<Response>(response).unwrap();
    }
}
