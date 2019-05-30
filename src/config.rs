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

#[derive(Deserialize)]
pub struct Usage {
    pub usage: String,
    pub example_input: String,
    pub example_output: String,
}

impl Usage {
    pub fn load(path: &str) -> Result<HashMap<String, Usage>, Box<Error>> {
        let content = fs::read_to_string(path)?;
        let o = ron::de::from_str(&content)?;

        Ok(o)
    }
}