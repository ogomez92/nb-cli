use serde::Deserialize;
use serde::Serialize;
use serde_json;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub url: String,
    pub token: String,
    pub last_channel: String,
}

impl Settings {
    /// new creates a new Settings struct with default values
    pub fn new() -> Settings {
        Settings {
            token: String::new(),
            url: String::new(),
            last_channel: String::new(),
        }
    }

    /// load loads the settings from the settings.json file
    pub fn load() -> Result<Settings, String> {
        let path = Path::new("settings.json");
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(_) => return Err("settings.json not found".to_string()),
        };

        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        match serde_json::from_str(&content) {
            Ok(settings) => Ok(settings),
            Err(_) => Err("Error while reading settings.json".to_string()),
        }
    }

    /// save saves the settings to the settings.json file
    pub fn save(&self) -> Result<(), String> {
        let path = Path::new("settings.json");
        let mut file = match File::create(&path) {
            Ok(file) => file,
            Err(_) => return Err("Error while creating settings.json".to_string()),
        };
        match file.write_all(serde_json::to_string_pretty(&self).unwrap().as_bytes()) {
            Ok(_) => Ok(()),
            Err(_) => Err("Error while writing settings.json".to_string()),
        }
    }

    pub fn initialize() -> Settings {
        match Settings::load() {
            Ok(settings) => {
                println!("Settings loaded: {:?}", settings);
                settings
            }
            Err(_error) => {
                let new_settings = Settings::new();
                match new_settings.save() {
                    Ok(_) => new_settings,
                    Err(error) => {
                        panic!("Failed to create settings: {:?}", error);
                    }
                }
            }
        }
    }

    pub fn set_url(&mut self, url: String) {
        self.url = url;
        self.save().expect("Failed to save URL");
    }

    pub fn set_token(&mut self, token: String) {
        self.token = token;
        self.save().expect("Failed to save token");
    }

    pub fn set_last_channel(&mut self, channel: String) {
        self.last_channel = channel;
        self.save().expect("Failed to save last channel");
    }

        pub fn get_url(&self) -> String {
        self.url.clone()
    }

    pub fn get_token(&self) -> String {
        self.token.clone()
    }

    pub fn get_last_channel(&self) -> String {
        self.last_channel.clone()
    }

    pub fn has_credentials(&self) -> bool {
        self.url != "" && self.token != ""
    }
    pub fn reset(&mut self) {
        self.url = String::new();
        self.token = String::new();
        self.last_channel = String::new();
        self.save().expect("Failed to reset settings");
    }
}
