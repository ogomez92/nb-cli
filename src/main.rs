mod types {
    pub mod settings;
}

pub mod api;

use crate::types::settings::Settings;

use api::check_token;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "nb-cli")]
#[command(author = "Oriol")]
#[command(version = "1.0.0")]
#[command(about = "Notebrook command line application", long_about = None)]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Login { url: String, token: String },
}

fn main() {
    let mut settings = Settings::initialize();
    let args = Cli::parse();

    match args.command {
        Commands::Login { url, token } => {
            match check_token(&url, &token) {
                Ok(_) => {
                    // put url and token into settings
                    settings.set_url(url);
                    settings.set_token(token)
                }
                Err(error) => {
                    eprintln!("Error logging in! {}", error);
                    std::process::exit(1);
                }
            }

            println!("Success!");
        }
    }
}
