extern crate discord;
extern crate regex;

use std::{
    error::{Error},
};


use discord::model::{
    Message,
};

use crate::{
    config::Usage,
};

pub trait BotFunction {
    fn usage(&self) -> &Usage;

    fn error_msg(&self, msg: &str) -> Result<(), String> {
        Err(format!("{}\n{}", msg, self.usage().usage()))
    }

    fn func(&mut self, args: &str, message: &Message) -> Result<(), Box<Error>>;
}

pub mod dice;