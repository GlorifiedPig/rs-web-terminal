
pub struct Command {
    command: String,
    description: String,
    callback: Box<dyn Fn(Vec<String>) -> String>,
}

pub fn create_commands() -> Vec<Command> {
    let mut commands: Vec<Command> = Vec::new();

    let help = |args: Vec<String>| { // FIXME
        let mut help_message: String = String::from("Available commands:\n");
        for command in commands.iter_mut() {
            help_message.push_str(&format!("{} - {}\n", command.command, command.description));
        }
        help_message
    };

    let help_command: Command = Command {
        command: "help".to_string(),
        description: "Shows this help message".to_string(),
        callback: Box::new(help),
    };

    commands.push(help_command);

    commands
}