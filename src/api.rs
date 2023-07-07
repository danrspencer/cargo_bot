use crate::model::response::Response;
use serde::Serialize;

const URL: &str = "https://api.openai.com/v1/chat/completions";
const TOKEN: &str = "*************";

pub async fn send_request<T: Serialize>(body: &T) -> Response {
    let client = reqwest::Client::new();

    let res = client
        .post(URL)
        .basic_auth("", Some(TOKEN.to_string()))
        .json(body)
        .send()
        .await
        .unwrap();

    let result = res.text().await.unwrap();

    serde_json::from_str(&result).unwrap()
}
