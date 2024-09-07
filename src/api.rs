use crate::types::channel::Channel;
use crate::types::channel::ChannelContainer;
use crate::types::settings::Settings;
use reqwest::blocking;
use reqwest::StatusCode;
use serde_json::json;
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
    env_logger::init();
    let client = blocking::Client::new();
    let resp = client
        .get(&format!("{}/channels/", settings.get_url()))
        .header("authorization", settings.get_token())
        .send();
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

pub fn create_channel(settings: &mut Settings, name: &String) -> Result<Channel, Box<dyn Error>> {
    let client = blocking::Client::new();
    let resp = client
        .post(&format!("{}/channels/", settings.get_url()))
        .header("authorization", settings.get_token())
        .json(&json!({ "name": name.clone() }))
        .send();
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
            let channel: Result<Channel, _> = resp.json();
            match channel {
                Ok(channel) => Ok(channel),
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

pub fn send_message(
    settings: &Settings,
    channel: &Channel,
    message: &String,
) -> Result<(), Box<dyn Error>> {
    let client = blocking::Client::new();
    let resp = client
        .post(&format!(
            "{}/channels/{}/messages/",
            settings.get_url(),
            channel.id
        ))
        .header("authorization", settings.get_token())
        .json(&json!({ "content": message.clone() }))
        .send();
    // Handle network error
    let resp = match resp {
        Ok(response) => response,
        Err(e) => {
            return Err(format!("Network error: {}", e).into());
        }
    };

    match resp.status() {
        StatusCode::OK => Ok(()),
        StatusCode::NOT_FOUND => Err("Error 404: Resource not found".into()),
        StatusCode::UNAUTHORIZED => Err("Error 401: Unauthorized".into()),
        _ => Err(format!("Unexpected status code: {}", resp.status()).into()),
    }
}
