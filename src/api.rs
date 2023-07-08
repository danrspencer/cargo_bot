use crate::model::response::Response;
use serde::Serialize;

const URL: &str = "https://api.openai.com/v1/chat/completions";
const TOKEN: &str = "********";

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
        let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let ext = if response.is_err() { "err" } else { "json" };
        let path = format!("{}/debug/{}.{}", root, timestamp, ext);

        println!("🤖 saving debug output to {}", path);

        let text = if let Ok(ref response) = response {
            serde_json::to_string_pretty(response).unwrap()
        } else {
            result
        };

        std::fs::create_dir_all(format!("{}/debug", root)).unwrap();
        std::fs::write(path, text).unwrap();
    }

    response.unwrap()
}
