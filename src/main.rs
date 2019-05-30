extern crate ipomoea_bot;
extern crate discord;

#[macro_use]
extern crate lazy_static;

extern crate regex;

use std::{
    collections::{
        HashMap,
        HashSet,
    },
};

use ipomoea_bot::{
    config::{Config, Usage},
    funcs::{
        self,
        BotFunction,
    },
};

use discord::Discord;
use discord::model::{
    Event,
};

use regex::Regex;

fn main() {
    let config = Config::load("config.ron")
        .expect("failed to load config file");
    
    let discord = Discord::from_bot_token(&config.token)
        .expect("failed to login");

    let usages = Usage::load("usages.ron")
        .expect("failed to load usages file");

    let mut functions: HashMap<&str, Box<BotFunction>> = HashMap::new();
    functions.insert("dice", Box::new(funcs::dice::Dice::new(&discord, &config)));

    // Establish and use a websocket connection
    let (mut connection, _) = discord.connect()
        .expect("failed to connect");
    
    println!("Ready.");
    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r"^!(?P<cmd>\w+)\s*(?P<args>.*)").unwrap();
                }
                
                if let Some(cap) = RE.captures(&message.content) {
                    let cmd = &cap["cmd"];
                    let args = &cap["args"];

                    let cmd_not_found = || {
                        let msg = format!("command '{}' is not found.", cmd);
                        println!("{}", &msg);
                        println!("With args: {}", args);

                        let _ = discord.send_message(message.channel_id, &msg, "", false);
                    };

                    match cmd {
                        "usage" => {
                            let cmds: HashSet<_> = args.split(" ").filter(|s| *s != "").collect();
                            for name in cmds.iter() {
                                if functions.contains_key(name) {
                                    match usages.get(*name) {
                                        Some(u) => {
                                            let usage = format!("Usage:```{}```", u.usage);
                                            let example = format!("Example:```{}``````{}```", u.example_input, u.example_output);
                                            let msg = format!("{}{}", usage, example);
                                            let _ = discord.send_message(message.channel_id, &msg, "", false);
                                        },
                                        None => cmd_not_found(),
                                    }
                                }
                            }
                        },
                        _ => {
                            match functions.get_mut(cmd) {
                                Some(f) => {
                                    if let Err(e) = f.func(args, &message) {
                                        println!("Error occured: {:?}", e);
                                        let _ = discord.send_message(message.channel_id, &format!("{}", e), "", false);
                                    }
                                },
                                None => cmd_not_found(),
                            }
                        }
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