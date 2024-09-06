use crate::types::channel::Channel;
use crate::types::channel::ChannelContainer;
use crate::types::settings::Settings;
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

pub fn get_channel_list(settings: &mut Settings) -> Result<Vec<Channel>, Box<dyn Error>> {
    let client = blocking::Client::new();
    let resp = client
        .get(&format!("{}/channels/", settings.get_url()))
        .header("authorization", settings.get_token())
        .send();
    println!("{:?}", resp);
    // Handle network error
    let resp = match resp {
        Ok(response) => response,
        Err(e) => {
            settings.reset();
            return Err(format!("Network error: {}", e).into());
        }
    };

    match resp.status() {
        StatusCode::OK => {
            // Print resp.json() and assign it o channels below
            
            let channels: Result<ChannelContainer, _> = resp.json();
            
            match channels {
                // Return the vector inside the container
                Ok(container) => Ok(container.channels),
                Err(e) => Err(format!("Failed to parse JSON: {}", e).into()),
            }
        }
        StatusCode::NOT_FOUND => {
            settings.reset();
            return Err("Error 404: Resource not found".into());
        }
        StatusCode::UNAUTHORIZED => {
            settings.reset();
            Err("Error 401: Unauthorized".into())
        }
        _ => Err(format!("Unexpected status code: {}", resp.status()).into()),
    }
}
