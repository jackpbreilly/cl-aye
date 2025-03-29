use crate::command::execute_command;

pub fn validate(config: &crate::config::Config) -> Result<(), Box<dyn std::error::Error>> {
    let output = execute_command(&config.command)?;

    if output.trim() == config.expected_output.trim() {
        Ok(())
    } else {
        Err(format!(
            "Validation failed: expected '{}', got '{}'",
            config.expected_output, output
        )
        .into())
    }
}
