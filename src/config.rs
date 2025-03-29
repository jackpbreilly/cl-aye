use serde::Deserialize;

use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub command: String,
    pub expected_output: String,
}

pub fn load(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let config: Config = serde_json::from_str(&fs::read_to_string(path)?)?;
    Ok(config)
}
