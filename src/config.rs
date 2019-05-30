extern crate ron;
use std::{
    fs,
    error::{Error},
};

#[derive(Deserialize)]
pub struct Config {
    pub token: String,
    pub dice_max: i32
}

impl Config {
    pub fn load(path: &str) -> Result<Config, Box<Error>> {
        let content = fs::read_to_string(path)?;
        let config : Config = ron::de::from_str(&content)?;

        Ok(config)
    }
}