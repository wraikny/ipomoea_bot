#![feature(box_syntax)]

extern crate ipomoea_bot;
extern crate discord;

extern crate regex;

use std::{
    rc::Rc,
    collections::{
        HashMap,
    },
};

use ipomoea_bot::{
    config::{Config, Usage},
    funcs::{
        dice::Dice,
        BotFunction,
    },
    message::{MessageHandler},
};

use discord::Discord;
use discord::model::{
    Event,
};

fn main() {
    let config = Config::load("config.ron")
        .expect("failed to load config file");

    
    let discord = Discord::from_bot_token(&config.token)
        .expect("failed to login");
    let discord = Rc::new(discord);
    
    let mut usages = Usage::load("usages.ron")
        .expect("failed to load usages file");
    
    let mut functions: HashMap<String, Box<BotFunction>> = HashMap::new();

    let dice = box Dice::new(usages.remove("dice").unwrap(), discord.clone(), &config);
    functions.insert("dice".to_owned(), dice);

    let mut message_handler = MessageHandler::new(discord.clone(), functions);

    // Establish and use a websocket connection
    let (mut connection, _) = discord.connect()
        .expect("failed to connect");
    
    println!("Ready.");
    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                message_handler.receive(message);
            }
            Ok(_) => {}
            Err(discord::Error::Closed(code, body)) => {
                println!("Gateway closed on us with code {:?}: {}", code, body);
                break
            }
            Err(err) => println!("Receive error: {:?}", err)
        }
    }
}