use std::{
    rc::Rc,
    collections::{
        HashMap,
        HashSet,
    },
};

use crate::{
    config::{Usage},
    funcs::{
        BotFunction,
    },
};

use discord::Discord;
use discord::model::{
    ChannelId,
    Message,
};

use regex::Regex;

pub struct MessageHandler {
    discord: Rc<Discord>,
    usages: HashMap<String, Usage>,
    functions: HashMap<String, Box<BotFunction>>,
}

impl MessageHandler {
    pub fn new(
        discord: Rc<Discord>,
        usages: HashMap<String, Usage>,
        functions: HashMap<String, Box<BotFunction>>
    ) -> Self {
        MessageHandler {
            discord, usages, functions
        }
    }

    fn send_message(&self, channel_id: ChannelId, msg: &str, nonce: &str, tts: bool) {
        let _ = self.discord.send_message(channel_id, msg, nonce, tts);
    }

    fn cmd_not_found(&self, channel_id: ChannelId, cmd: &str, args: &str) {
        let msg = format!("command '{}' is not found.", cmd);
        println!("{}", &msg);
        println!("With args: {}", args);

        self.send_message(channel_id, &msg, "", false);
    }

    fn usage(&self, channel_id: ChannelId, cmd: &str, args: &str) {
        let cmds: HashSet<_> = args.split(" ").filter(|s| *s != "").collect();
        for name in cmds.iter() {
            if self.functions.contains_key(*name) {
                match self.usages.get(*name) {
                    Some(u) => {
                        let usage = format!("Usage:```{}```", &u.usage);
                        let example = format!("Example:```{}``````{}```", &u.example_input, &u.example_output);
                        let msg = format!("{}{}", usage, example);
                        self.send_message(channel_id, &msg, "", false);
                    },
                    None => {
                        let msg = format!("Command {} exists, but its usage doesn't exist", name);
                        println!("{}", &msg);
                        self.send_message(channel_id, &msg, "", false);
                    },
                }
            } else {
                self.cmd_not_found(channel_id, cmd, args);
            }
        }
    }

    fn list(&self, channel_id: ChannelId) {
        let functions = self.functions.keys()
            .map(|k| format!("{}", k))
            .collect::<Vec<_>>()
            .join("\n");
        self.send_message(channel_id, &functions, "", false);
    }

    fn functions(&mut self, message: &Message, cmd: &str, args: &str) {
        match self.functions.get_mut(cmd) {
            Some(f) => {
                if let Err(e) = f.func(args, message) {
                    println!("Error occured: {:?}", e);
                    self.send_message(message.channel_id, &format!("{}", e), "", false);
                }
            },
            None => self.cmd_not_found(message.channel_id, cmd, args),
        }
    }

    pub fn receive(&mut self, message: Message) {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^!(?P<cmd>\w+)\s*(?P<args>.*)").unwrap();
        }

        if let Some(cap) = RE.captures(&message.content) {
            let cmd = &cap["cmd"];
            let args = &cap["args"];

            match cmd {
                "usage" => self.usage(message.channel_id, cmd, args),
                "list" => self.list(message.channel_id),
                _ => self.functions(&message, cmd, args),
            }
        }
    }
}