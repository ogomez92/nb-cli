use clap::Subcommand;
#[derive(Subcommand)]
pub enum Commands {
    Login { url: String, token: String },
    Lsc,
}
