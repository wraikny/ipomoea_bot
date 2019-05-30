use regex::Regex;

use discord::Discord;
use discord::model::{
    Message,
};

use std::error::Error;

use rand::Rng;

use crate::config::{Config};
use super::BotFunction;

pub struct Dice<'a> {
    rng: rand::prelude::ThreadRng,

    discord: &'a Discord,
    dice_max: i32,
}

impl<'a> Dice<'a> {
    pub fn new(discord: &'a Discord, config: &Config) -> Self {
        Dice {
            rng: rand::thread_rng(),
            discord: discord,
            dice_max: config.dice_max,
        }
    }
}

impl<'a> BotFunction for Dice<'a> {
    fn usage(&self) -> String {
        r"!dice (Int)d(Int) ..".to_owned()
    }

    fn example(&self) -> (String, String) {
        ("!dice 1d6".to_owned()
        , "1d6: [5] => 5".to_owned())
    }

    fn func(&mut self, args: &str, message: &Message) -> Result<(), Box<Error>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?P<count>\d+)[dD](?P<roll>\d+)").unwrap();
        }

        let args : Vec<&str> = args.split(" ").filter(|s| *s != "").collect();

        if args.len() == 0 {
            let _ = Err("'!dice' requires one or more arguments.")?;
        }

        let msg = args.into_iter()
            .map(|arg| {
                let r : Result<String, Box<Error>> = (|| {
                    let cap = RE.captures(arg).ok_or("Failed to parse into MdN")?;
                    let count: i32 = (&cap["count"]).parse()?;

                    if count > self.dice_max {
                        let msg = format!("dice count must less than {}", self.dice_max);
                        let _ = Err(msg)?;
                    }

                    let roll: i32 = (&cap["roll"]).parse()?;
                    let rolls: Vec<i32> = (0..count).map(|_| self.rng.gen_range(1, roll + 1)).collect();

                    let sum: i32 = rolls.iter().fold(0, |sum, i| sum + i);
                    let rolls_str = rolls.iter()
                        .map(|r| r.to_string())
                        .collect::<Vec<String>>().join(", ");

                    Ok(format!("[{}] => {}", rolls_str, sum))
                })();
                (arg, r)
            })
            .map(|(arg, r)| {
                let s = match r {
                    Ok(s) => s,
                    Err(e) => format!("{}", e),
                };
                format!("{}: {}", arg, s)
            })
            .collect::<Vec<String>>()
            .join("\n");

        let _ = self.discord.send_message(message.channel_id, &msg, "", false)?;

        Ok(())
    }
}