use serde_json;
use serde::Serialize;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub token: String,
    pub url: String,
}

impl Settings {
    /// new creates a new Settings struct with default values
    pub fn new() -> Settings {
        Settings {
            token: String::new(),
            url: String::new(),
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
}
