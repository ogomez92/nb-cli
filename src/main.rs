mod types;
mod enums;
mod api;

use crate::types::settings::Settings;
use crate::enums::commands;
use clap::Parser;
use std::fmt::format;

#[derive(Parser)]
#[command(name = "nb-cli")]
#[command(author = "Oriol")]
#[command(version = "1.0.0")]
#[command(about = "Notebrook command line application", long_about = None)]

struct Cli {
    #[command(subcommand)]
    command: commands::Commands,
}


fn main() {
    let mut settings = Settings::initialize();
    let args = Cli::parse();

    match args.command {
        commands::Commands::Login { url, token } => {
            match api::check_token(&url, &token) {
                Ok(_) => {
                    // put url and token into settings
                    settings.set_url(url);
                    settings.set_token(token);
                }
                Err(error) => {
                    eprintln!("Error logging in! {}", error);
                    std::process::exit(1);
                }
            }
            println!("Success!");
        }
        commands::Commands::Lsc if !settings.has_credentials() => {
            eprintln!("No credentials are set, please use login <url> <token> before performing any other commands.");
            std::process::exit(1);
        }
        commands::Commands::Lsc => {
            let channels = api::get_channel_list(&mut settings);
            match channels {
                Ok(channels) => {
                    if channels.is_empty() {
                        println!("You don't have any channels in your Notebrook. You can create one by using the send command to send a note and specifying a channel via the `-c` or `--channel` flag.");
                    } else {
                        for channel in channels {
                            println!("{}", channel);
                        }
                    }
                }
                Err(error) => {
                    eprintln!("Error listing channels. {}", error);
                    std::process::exit(1)
                }
            }
        }
    }
}
