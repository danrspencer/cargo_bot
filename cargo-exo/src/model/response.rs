use cargo_exo_functions::{explain::ExplainParams, update_files::UpdateFilesParams};
use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize};

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
    // TODO - Force these enums to match up with the provided function names to GPT
    #[serde(deserialize_with = "deserialize_nested_json")]
    UpdateFile(UpdateFilesParams),
    #[serde(deserialize_with = "deserialize_nested_json")]
    Explain(ExplainParams),
}

fn deserialize_nested_json<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    serde_json::from_str(&s).map_err(serde::de::Error::custom)
}

#[cfg(test)]
mod test {
    use cargo_exo_functions::update_files::{FileUpdate, LineAction, LineUpdate};
    use serde_json::json;

    use super::*;

    #[test]
    fn it_parses_a_response() {
        let fn_name = stringify!(update_file);
        let args = serde_json::to_string(&UpdateFilesParams {
            files: vec![FileUpdate {
                cause: "oh no - something broke".to_string(),
                file: "hello.ts".to_string(),
                lines: vec![LineUpdate {
                    line_no: 10,
                    content: Some("test".to_string()),
                    action: LineAction::Replace,
                }],
            }],
        })
        .unwrap();

        let response = json!({
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
                  "name": fn_name,
                  "arguments": args
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
        });

        let response_str = serde_json::to_string(&response).unwrap();

        serde_json::from_str::<Response>(&response_str).unwrap();
    }
}
