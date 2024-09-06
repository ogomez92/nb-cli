use reqwest::blocking;
use reqwest::StatusCode;
use std::error::Error;

pub fn check_token(api_url: &str, auth_token: &str) -> Result<String, Box<dyn Error>> {
    let client = blocking::Client::new();

    let resp = client
        .get(format!("{}/check-token", api_url))
        .header("authorization", auth_token)
        .send()?;

    match resp.status() {
        StatusCode::OK => {
            let body = resp.text()?;
            Ok(body)
        }
        StatusCode::NOT_FOUND => Err("Error 404: Resource not found".into()),
        StatusCode::UNAUTHORIZED => Err("Error 401: Unauthorized".into()),
        _ => Err(format!("Unexpected status code: {}", resp.status()).into()),
    }
}
