
use std::sync::Mutex;

type CommandCallback = fn(Vec<String>) -> String;

pub struct Command {
    pub command: &'static str,
    pub description: &'static str,
    pub callback: CommandCallback,
}

pub static COMMANDS: Mutex<Vec<Command>> = Mutex::new(Vec::new());

pub fn create_commands() {
    let help: CommandCallback = |_args: Vec<String>| {
        let mut help_message: String = String::from("Available commands:\n");
        for command in COMMANDS.lock().unwrap().iter() {
            help_message.push_str(&format!("{} - {}\n", command.command, command.description));
        }
        help_message
    };

    let help_command: Command = Command {
        command: "help",
        description: "Shows this help message",
        callback: help,
    };

    // get current date command
    // maybe a basic regex parser

    COMMANDS.lock().unwrap().push(help_command);
}