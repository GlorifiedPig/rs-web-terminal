
use std::{sync::RwLock, time::SystemTime};

type CommandCallback = fn(Vec<String>) -> String;

pub struct Command {
    pub command: &'static str,
    pub description: &'static str,
    pub callback: CommandCallback,
}

pub static COMMANDS: RwLock<Vec<Command>> = RwLock::new(Vec::new());

pub fn push_command(command: Command) {
    match COMMANDS.write() {
        Ok(mut commands) => commands.push(command),
        Err(_) => println!("Unable to push command")
    }
}

pub fn create_commands() {
    let help: Command = Command {
        command: "help",
        description: "Shows this help message",
        callback: |args: Vec<String>| {
            let commands = COMMANDS.read().unwrap();

            if args.len() > 0 {
                let command: Option<&Command> = commands.iter().find(|c| c.command == args[0]);
                match command {
                    Some(c) => format!("{} - {}", c.command, c.description),
                    None => format!("Command {} not found", args[0])
                }
            } else {
                let mut help_message: String = String::from("Available commands:");
                for command in commands.iter() {
                    help_message.push_str(&format!("\n{} - {}", command.command, command.description));
                }
                help_message
            }
        },
    };

    let date: Command = Command {
        command: "date",
        description: "Shows the current UTC date",
        callback: |_| timestamp_to_date(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()),
    };

    push_command(help);
    push_command(date);
}

fn timestamp_to_date(timestamp: u64) -> String {
    let year = 1970 + (timestamp / 31556952);
    let month = ((timestamp % 31556952) / 2629746) + 1;
    let day = (timestamp % 2629746) / 86400;
    let hour = (timestamp % 86400) / 3600;
    let minute = (timestamp % 3600) / 60;
    let second = timestamp % 60;

    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        year, month, day, hour, minute, second
    )
}