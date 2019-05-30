extern crate ron;
use std::{
    fs,
    error::{Error},
    collections::HashMap,
};

#[derive(Deserialize)]
pub struct Config {
    pub token: String,
    pub dice_max: i32
}

impl Config {
    pub fn load(path: &str) -> Result<Config, Box<Error>> {
        let content = fs::read_to_string(path)?;
        let o = ron::de::from_str(&content)?;

        Ok(o)
    }
}

#[derive(Clone, Deserialize)]
pub struct Usage {
    usage: String,
    example_input: String,
    example_output: String,
}

impl Usage {
    pub fn load(path: &str) -> Result<HashMap<String, Usage>, Box<Error>> {
        let content = fs::read_to_string(path)?;
        let o = ron::de::from_str(&content)?;

        Ok(o)
    }

    pub fn usage(&self) -> String {
        format!("Usage:```{}```", &self.usage)
    }

    pub fn example(&self) -> String {
        format!("Example:```{}``````{}```", &self.example_input, &self.example_output)
    }
}