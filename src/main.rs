
mod miniflux;
mod libreddit_config_sync;

use clap::{Parser, Subcommand};
use miniflux::MinifluxContext;

/// Simple cli to perform various miniflux-related tasks.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand, Debug)]
enum Commands {
    LibredditConfigSync
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::LibredditConfigSync => {
            let secret = rpassword::prompt_password("Secret: ").unwrap();
            let miniflux_context = MinifluxContext::new(secret);
            libreddit_config_sync::perform(miniflux_context);
        }
    }
}

