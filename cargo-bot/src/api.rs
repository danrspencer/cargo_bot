use crate::model::response::Response;
use serde::Serialize;

const URL: &str = "https://api.openai.com/v1/chat/completions";
const TOKEN: &str = "*****";

const DEBUG: bool = true;

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
    let response = serde_json::from_str(&result);

    if DEBUG {
        let timestamp = chrono::Local::now().format("%Y%m%d%H%M%S");
        let ext = if response.is_err() { "err" } else { "json" };
        let path = "/tmp/cargo_bot_debug";
        let filepath = format!("{}/{}.{}", path, timestamp, ext);

        println!("🤖 saving debug output to {}", filepath);

        let text = if let Ok(ref response) = response {
            serde_json::to_string_pretty(response).unwrap()
        } else {
            result
        };

        std::fs::create_dir_all(path).expect("Couldn't create debug directory");
        std::fs::write(&filepath, text).expect("Couldn't write response file");
    }

    response.unwrap()
}
