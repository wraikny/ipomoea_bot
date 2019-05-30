use regex::Regex;

use discord::Discord;
use discord::model::{
    Message,
};

use std::{rc::Rc, error::Error};

use rand::Rng;

use crate::config::{Config};
use super::BotFunction;

use crate::{
    config::Usage,
};

pub struct Dice {
    usage: Usage,
    rng: rand::prelude::ThreadRng,

    discord: Rc<Discord>,
    dice_max: i32,
}

impl Dice {
    pub fn new(usage: Usage, discord: Rc<Discord>, config: &Config) -> Self {
        Dice {
            usage,
            rng: rand::thread_rng(),
            discord: discord,
            dice_max: config.dice_max,
        }
    }
}

impl BotFunction for Dice {
    fn usage(&self) -> &Usage {
        &self.usage
    }

    fn func(&mut self, args: &str, message: &Message) -> Result<(), Box<Error>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?P<count>\d+)[dD](?P<roll>\d+)").unwrap();
        }

        let args : Vec<&str> = args.split(" ").filter(|s| *s != "").collect();

        if args.len() == 0 {
            self.error_msg("'!dice' requires one or more arguments.")?;
        }

        let results : Vec<_> = args.into_iter()
            .map(|arg| {
                let r : Result<(i32, String), Box<Error>> = (|| {
                    let cap = RE.captures(arg).ok_or("Failed to parse")?;
                    let count: i32 = (&cap["count"]).parse()?;

                    if count > self.dice_max {
                        Err(format!("Dice count must less than {}", self.dice_max))?;
                    }

                    let roll: i32 = (&cap["roll"]).parse()?;

                    let rolls: Vec<i32> = (0..count).map(|_| self.rng.gen_range(1, roll + 1)).collect();

                    let sum: i32 = rolls.iter().fold(0, |sum, i| sum + i);
                    let rolls_str = rolls.iter()
                        .map(|r| r.to_string())
                        .collect::<Vec<String>>().join(", ");

                    Ok((sum, format!("[{}] => {}", rolls_str, sum)))
                })();
                (arg, r.map_err(|e| format!("{}", e)))
            }).collect();

        if results.clone().iter().any(|(_, r)| r.is_err()) {
            let msg = 
                results
                .into_iter()
                .filter(|(_, r)| r.is_err())
                .map(|(arg, e)| {
                    format!("{}: {}", arg, e.unwrap_err())
                }).collect::<Vec<_>>().join("\n");

            self.error_msg(&msg)?;
        } else {
            let mut sum_all = 0;

            let length = results.len();
            
            let msg =
                results
                .into_iter()
                .map(|(arg, r)| {
                    let (sum, s) = r.unwrap();
                    sum_all += sum;
                    format!("{}: {}", arg, s)
                }).collect::<Vec<_>>().join("\n");

            let msg = if length > 1 { msg + &format!("\n=> {}\n", sum_all) } else { msg };
            
            self.discord.send_message(message.channel_id, &msg, "", false)?;
        }

        Ok(())
    }
}