extern crate ipomoea_bot;
extern crate discord;

#[macro_use]
extern crate lazy_static;

extern crate regex;

use std::{
    error::{Error},
    collections::{
        HashMap,
    },
};

use ipomoea_bot::{
    config::Config,
    funcs,
};

use discord::Discord;
use discord::model::{
    Event,
    Message,
};

use regex::Regex;

fn main() {
    let config = Config::load("token.ron")
        .expect("failed to load config");
    
    let discord = Discord::from_bot_token(&config.token)
        .expect("failed to login");

    let mut functions: HashMap<&str, Box<Fn(&Vec<&str>, &Message, &Discord) -> Result<(), Box<Error>>>> = HashMap::new();
    functions.insert("dice", Box::new(funcs::dice::dice));
    let functions = functions;

    // Establish and use a websocket connection
    let (mut connection, _) = discord.connect()
        .expect("failed to connect");
    
    println!("Ready.");
    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r"!(?P<cmd>\w+)\s*(?P<args>.*)").unwrap();
                }
                if let Some(cap) = RE.captures(&message.content) {
                    let cmd = &cap["cmd"];
                    let args: Vec<&str> = (&cap["args"]).split(" ").collect();
                    match functions.get(cmd) {
                        Some(func) => {
                            if let Err(e) = func(&args, &message, &discord) {
                                println!("Error occured: {:?}", e);
                            }
                        },
                        None => {
                            println!("Does not exist command: {}", cmd);
                            println!("With args: {}", args.join(" "));
                        },
                    }
                }
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