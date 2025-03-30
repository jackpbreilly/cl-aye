mod command;
mod config;
mod validator;

use crate::config::Config;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    config: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let config_path = args.config;
    let config: Config = config::load(&config_path)?;
    validator::validator(&config)?;

    Ok(())
}
