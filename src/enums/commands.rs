use clap::Subcommand;
#[derive(Subcommand)]
pub enum Commands {
    Login {
        url: String,
        token: String,
    },
    Lsc,
    Send {
        message: String,
        #[clap(short, long)]
        channel: Option<String>,
        #[clap(short, long)]
        yes: bool,
    },
    Read {
        #[clap(short, long)]
        channel: Option<String>,
    },
}
