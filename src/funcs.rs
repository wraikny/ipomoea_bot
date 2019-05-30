extern crate discord;
extern crate regex;

use std::{
    error::{Error},
};


use discord::model::{
    Message,
};

pub trait BotFunction {
    fn usage(&self) -> String;
    fn example(&self) -> (String, String);
    fn func(&mut self, args: &str, message: &Message) -> Result<(), Box<Error>>;
}

pub mod dice;