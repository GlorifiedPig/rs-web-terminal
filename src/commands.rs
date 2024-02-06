
use std::sync::RwLock;

type CommandCallback = fn(Vec<String>) -> String;

pub struct Command {
    pub command: &'static str,
    pub description: &'static str,
    pub callback: CommandCallback,
}

pub static COMMANDS: RwLock<Vec<Command>> = RwLock::new(Vec::new());

pub fn push_command(command: Command) {
    match COMMANDS.write() {
        Ok(mut commands) => {
            commands.push(command);
        },
        Err(_) => {
            println!("Unable to push command");
        }
    }
}

pub fn create_commands() {
    let help: Command = Command {
        command: "help",
        description: "Shows this help message",
        callback: |_args: Vec<String>| {
            let mut help_message: String = String::from("Available commands:");
            for command in COMMANDS.read().unwrap().iter() {
                help_message.push_str(&format!("\n{} - {}", command.command, command.description));
            }
            help_message
        },
    };

    let date: Command = Command {
        command: "date",
        description: "Shows the current date",
        callback: |_args: Vec<String>| {
            String::from("Todo")
        },
    };

    let regex = Command {
        command: "regex",
        description: "Matches a regex pattern",
        callback: |_args: Vec<String>| {
            String::from("Todo")
        },
    };

    push_command(help);
    push_command(date);
    push_command(regex);
}