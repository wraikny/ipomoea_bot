use regex::Regex;

use discord::Discord;
use discord::model::{
    Message,
};

use std::error::Error;

use rand::Rng;

pub fn dice(args: &Vec<&str>, message: &Message, discord: &Discord) -> Result<(), Box<Error>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<count>\d+)[dD](?P<roll>\d+)").unwrap();
    }

    let mut rng = rand::thread_rng();
    
    let msg = args.into_iter()
        .map(|arg| {
            let r : Result<String, Box<Error>> = (|| {
                let cap = RE.captures(arg).ok_or("Failed to parse")?;
                let count: i32 = (&cap["count"]).parse()?;
                let roll: i32 = (&cap["roll"]).parse()?;
                let rolls: Vec<i32> = (0..count).map(|_| rng.gen_range(1, roll + 1)).collect();
                let sum: i32 = rolls.iter().fold(0, |sum, i| sum + i);
                let rolls_str = rolls.iter().map(|r| r.to_string()).collect::<Vec<String>>().join(", ");
                Ok(format!("[{}] => {}", rolls_str, sum))
            })();
            (arg, r)
        })
        .map(|(arg, r)| {
            let s = match r {
                Ok(s) => s,
                Err(e) => format!("{:?}", e),
            };
            format!("{}: {}", arg, s)
        })
        .collect::<Vec<String>>()
        .join("\n");

    let _ = discord.send_message(message.channel_id, &msg, "", false)?;

    Ok(())
}