mod types {
    pub mod settings;
}
use crate::types::settings::Settings;

fn main() {
    // attempt to load the settings file
    let settings = Settings::load();
    match settings {
        Ok(settings) => {
            println!("Settings loaded: {:?}", settings);
        }
        Err(error) => {
            println!("Error while loading settings: {}", error);
        }
    }
}
