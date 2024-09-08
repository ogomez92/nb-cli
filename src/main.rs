mod api;
mod enums;
mod types;

use crate::enums::commands;
use crate::types::channel::Channel;
use crate::types::settings::Settings;
use clap::Parser;
use types::channel;

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

        commands::Commands::Send { message, channel } => {
            if settings.get_last_channel() == "" && channel.is_none() {
                eprintln!("No channel is specified. Please specify a channel with the -c or --channel flag.");
                std::process::exit(1);
            }

            let channel = channel.unwrap_or(settings.get_last_channel());

            let mut channels = api::get_channel_list(&mut settings);
            match channels {
                Ok(ref mut channels) => {
                    let mut channel_exists = false;
                    for ch in &mut *channels {
                        if ch.name == channel {
                            channel_exists = true;
                        }
                    }
                    if !channel_exists {
                        println!(
                            "Channel {} does not exist. Do you want to create it? (y/n)",
                            channel
                        );
                        let mut response = String::new();
                        std::io::stdin().read_line(&mut response).unwrap();
                        if response.trim() != "y" {
                            std::process::exit(0);
                        } else {
                            let channel_from_server: Channel =
                                api::create_channel(&mut settings, &channel)
                                    .expect("Unable to create channel, exiting.");
                            channels.push(channel_from_server);
                            println!("Channel {} created.", channel);
                        }
                    }
                }
                Err(error) => {
                    eprintln!("Error listing channels. {}", error);
                    std::process::exit(1);
                }
            }

            settings.set_last_channel(channel.clone());
            let unwrapped_channels = &channels.unwrap();
            let found_channel = channel::find_channel_by_name(&&unwrapped_channels, &channel)
                .expect("Error was found when getting ID of channel");
            let result = api::send_message(&mut settings, found_channel, &message);
            match result {
                Ok(_) => {
                    println!("Message sent to channel {}!", channel);
                }
                Err(error) => {
                    eprintln!("Error sending message! {}", error);
                    std::process::exit(1);
                }
            }
        }

        commands::Commands::Read { channel } => {
            if settings.get_last_channel() == "" && channel.is_none() {
                eprintln!("No channel is specified. Please specify a channel with the -c or --channel flag.");
                std::process::exit(1);
            }

            let channel = channel.unwrap_or(settings.get_last_channel());

            let mut channels = api::get_channel_list(&mut settings);
            match channels {
                Ok(ref mut channels) => {
                    let mut channel_exists = false;
                    for ch in &mut *channels {
                        if ch.name == channel {
                            channel_exists = true;
                        }
                    }

                    if !channel_exists {
                        println!(
                            "The channel you are trying to read, {}, does not exist. Exiting.",
                            channel
                        );
                        std::process::exit(1);
                    }
                }
                Err(error) => {
                    eprintln!("Error listing channels. {}", error);
                    std::process::exit(1);
                }
            }

            settings.set_last_channel(channel.clone());
            let unwrapped_channels = &channels.unwrap();
            let found_channel = channel::find_channel_by_name(&&unwrapped_channels, &channel)
                .expect("Error was found when getting ID of channel");
            let messages = api::read_channel(&mut settings, found_channel);
            match messages {
                Ok(messages) => {
                    if messages.is_empty() {
                        println!("No messages in channel {}.", channel);
                    } else {
                        for message in messages {
                            println!("{}", message);
                        }
                    }
                }
                Err(error) => {
                    eprintln!("Error reading messages. {}", error);
                    std::process::exit(1);
                }
            }
        }
    }
}
