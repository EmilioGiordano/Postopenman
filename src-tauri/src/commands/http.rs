use crate::models::{HttpResponse, KeyValuePair};
use std::time::Instant;

#[tauri::command]
pub async fn send_request(
    method: String,
    url: String,
    headers: Vec<KeyValuePair>,
    params: Vec<KeyValuePair>,
    body: String,
    body_type: String,
    auth_type: String,
    auth_data: String,
) -> Result<HttpResponse, String> {
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::limited(10))
        .build()
        .map_err(|e| e.to_string())?;

    let req_method = match method.to_uppercase().as_str() {
        "GET" => reqwest::Method::GET,
        "POST" => reqwest::Method::POST,
        "PUT" => reqwest::Method::PUT,
        "DELETE" => reqwest::Method::DELETE,
        "PATCH" => reqwest::Method::PATCH,
        "HEAD" => reqwest::Method::HEAD,
        "OPTIONS" => reqwest::Method::OPTIONS,
        other => return Err(format!("unsupported HTTP method: {other}")),
    };

    let mut request_builder = client.request(req_method, &url);

    let enabled_params: Vec<(&str, &str)> = params
        .iter()
        .filter(|p| p.enabled && !p.key.is_empty())
        .map(|p| (p.key.as_str(), p.value.as_str()))
        .collect();

    if !enabled_params.is_empty() {
        request_builder = request_builder.query(&enabled_params);
    }

    for header in &headers {
        if header.enabled && !header.key.is_empty() {
            request_builder = request_builder.header(&header.key, &header.value);
        }
    }

    match auth_type.as_str() {
        "bearer" => {
            let auth: serde_json::Value =
                serde_json::from_str(&auth_data).map_err(|e| e.to_string())?;
            let token = auth["token"].as_str().unwrap_or_default();
            request_builder = request_builder.bearer_auth(token);
        }
        "basic" => {
            let auth: serde_json::Value =
                serde_json::from_str(&auth_data).map_err(|e| e.to_string())?;
            let username = auth["username"].as_str().unwrap_or_default();
            let password = auth["password"].as_str().unwrap_or_default();
            request_builder = request_builder.basic_auth(username, Some(password));
        }
        _ => {}
    }

    match body_type.as_str() {
        "json" => {
            let json_value: serde_json::Value =
                serde_json::from_str(&body).map_err(|e| format!("invalid JSON body: {e}"))?;
            request_builder = request_builder
                .header("Content-Type", "application/json")
                .json(&json_value);
        }
        "text" => {
            request_builder = request_builder
                .header("Content-Type", "text/plain")
                .body(body);
        }
        "form-urlencoded" => {
            let form_pairs: Vec<KeyValuePair> =
                serde_json::from_str(&body).map_err(|e| format!("invalid form body: {e}"))?;
            let form_data: Vec<(&str, &str)> = form_pairs
                .iter()
                .filter(|p| p.enabled && !p.key.is_empty())
                .map(|p| (p.key.as_str(), p.value.as_str()))
                .collect();
            request_builder = request_builder.form(&form_data);
        }
        _ => {}
    }

    let start = Instant::now();
    let response = request_builder.send().await.map_err(|e| e.to_string())?;
    let elapsed = start.elapsed().as_millis() as u64;

    let status = response.status().as_u16();
    let status_text = response
        .status()
        .canonical_reason()
        .unwrap_or("Unknown")
        .to_string();

    let response_headers: Vec<KeyValuePair> = response
        .headers()
        .iter()
        .map(|(k, v)| KeyValuePair {
            key: k.to_string(),
            value: v.to_str().unwrap_or_default().to_string(),
            enabled: true,
        })
        .collect();

    let body_bytes = response.bytes().await.map_err(|e| e.to_string())?;
    let size_bytes = body_bytes.len() as u64;
    let body_text = String::from_utf8_lossy(&body_bytes).to_string();

    Ok(HttpResponse {
        status,
        status_text,
        headers: response_headers,
        body: body_text,
        time_ms: elapsed,
        size_bytes,
    })
}
