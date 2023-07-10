use crate::model;
use crate::model::response::Response;
use serde::Serialize;

const URL: &str = "https://api.openai.com/v1/chat/completions";

const DEBUG: bool = true;

pub async fn send_request<T: Serialize>(
    body: &T,
    api_key: String,
) -> Result<Response, model::error::Error> {
    let client = reqwest::Client::new();

    let res = client
        .post(URL)
        .basic_auth("", Some(api_key))
        .json(body)
        .send()
        .await
        .unwrap();

    let success = res.status().is_success();
    let result = res.text().await.unwrap();

    if !success {
        let error = serde_json::from_str::<model::error::ErrorResponse>(&result).expect(&result);

        return Err(error.error);
    }

    let response = serde_json::from_str::<model::response::Response>(&result);

    if DEBUG {
        let timestamp = chrono::Local::now().format("%Y%m%d%H%M%S");
        let path = "/tmp/cargo_bot_debug";
        let filepath = format!("{}/{}.json", path, timestamp);

        println!("🤖 saving debug output to {}", filepath);

        let text = if let Ok(ref response) = response {
            serde_json::to_string_pretty(response).unwrap()
        } else {
            result
        };

        std::fs::create_dir_all(path).expect("Couldn't create debug directory");
        std::fs::write(&filepath, text).expect("Couldn't write response file");
    }

    Ok(response.unwrap())
}
