// src/main.rs
mod cli;
mod config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::get_cli_args();
    let suite = config::TestSuite::from_file(&args.config)?;

    Ok(())
}
