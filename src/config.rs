use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandStep {
    #[serde(rename = "Step")]
    pub step: String,
    #[serde(rename = "Command")]
    pub command: String,
    #[serde(rename = "Expected Output")]
    pub expected_output: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnvironmentSetup {
    #[serde(rename = "Docker File")]
    pub docker_file: String,
    #[serde(rename = "Test Setup")]
    pub test_setup: Vec<CommandStep>,
    #[serde(rename = "Test Teardown")]
    pub test_teardown: Vec<CommandStep>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Testcase {
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Enviroment Setup")]
    pub environment_setup: EnvironmentSetup,
    #[serde(rename = "Test Steps")]
    pub test_steps: Vec<CommandStep>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestSuite {
    #[serde(rename = "Suite Name")]
    pub suite_name: String,
    #[serde(rename = "Testcases")]
    pub testcases: HashMap<String, Testcase>,
}

impl TestSuite {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<TestSuite, Box<dyn Error>> {
        let data = fs::read_to_string(path)?;
        let suite: TestSuite = serde_json::from_str(&data)?;
        Ok(suite)
    }
}
