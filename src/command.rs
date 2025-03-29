use std::process::Command;

pub fn execute_command(command: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("sh").arg("-c").arg(command).output()?;

    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?)
    } else {
        Err(String::from_utf8(output.stderr)?.into())
    }
}
