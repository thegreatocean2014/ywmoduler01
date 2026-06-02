
// use tauri::command;
use reqwest::Client;
use serde_json::Value;

#[tauri::command]
pub async fn make_http_request(
    method: String,
    url: String,
    body: Option<Value>,
) -> Result<Value, String> {
    let client = Client::new();

    let request = match method.as_str() {
        "GET" => client.get(&url),
        "POST" => {
            let mut req = client.post(&url);
            if let Some(b) = body {
                req = req.json(&b);
            }
            req
        }
        "PUT" => {
            let mut req = client.put(&url);
            if let Some(b) = body {
                req = req.json(&b);
            }
            req
        }
        "DELETE" => client.delete(&url),
        _ => return Err("Unsupported HTTP method".to_string()),
    };

    match request.send().await {
        Ok(response) => {
            match response.json().await {
                Ok(data) => Ok(data),
                Err(e) => Err(format!("Failed to parse response: {}", e)),
            }
        }
        Err(e) => Err(format!("HTTP request failed: {}", e)),
    }
}


