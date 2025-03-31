use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(long)]
    pub config: String,
}

pub fn get_cli_args() -> Cli {
    Cli::parse()
}
