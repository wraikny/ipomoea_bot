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
    config::Config,
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
        .expect("failed to load config");
    
    let discord = Discord::from_bot_token(&config.token)
        .expect("failed to login");

    let mut functions: HashMap<&str, Box<BotFunction>> = HashMap::new();
    functions.insert("dice", Box::new(funcs::dice::Dice::new(&discord, &config)));
    // let functions = functions;

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
                                match functions.get(name) {
                                    Some(f) => {
                                        let usage = format!("Usage:```{}```", f.usage());
                                        let ex = f.example();
                                        let example = format!("Example:```{}``````{}```", ex.0, ex.1);
                                        let msg = format!("{}{}", usage, example);
                                        let _ = discord.send_message(message.channel_id, &msg, "", false);
                                    },
                                    None => cmd_not_found(),
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