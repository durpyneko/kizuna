mod agent;
mod service;
mod types;

use types::Command;

pub const COMMANDS: &[Command] = &[agent::COMMAND, service::COMMAND];

impl Command {
    pub fn matches(&self, input: &str) -> bool {
        self.name == input || self.aliases.contains(&input)
    }

    pub fn print_help(&self) {
        print_help(Some(self));
    }

    pub fn usage(&self) -> String {
        match self.subcommands {
            Some(subs) if !subs.is_empty() => {
                let names = subs.iter().map(|s| s.name).collect::<Vec<_>>().join("|");
                format!("kizuna {} <{}>", self.name, names)
            }
            _ => format!("kizuna {}", self.name),
        }
    }
}

pub fn find_command(input: &str) -> Option<&'static Command> {
    COMMANDS.iter().find(|cmd| cmd.matches(input))
}

pub async fn handler(args: Vec<String>) -> std::io::Result<()> {
    let Some(cmd) = args.get(1) else {
        print_help(None);
        return Ok(());
    };

    let Some(command) = find_command(cmd) else {
        log::warn!("Unknown command: {}", cmd);
        print_help(None);
        return Ok(());
    };

    let rest: Vec<String> = args.into_iter().skip(2).collect();
    (command.handler)(rest).await
}

pub fn print_help(command: Option<&Command>) {
    match command {
        None => {
            println!("Usage:");
            println!("  kizuna <command>\n");

            // println!("Commands:");

            println!("{:<14} {:<20} {}", "Command", "Aliases", "Description");
            println!("{:-<10}    {:-<16}     {:-<40}", "", "", "");

            for cmd in COMMANDS {
                println!(
                    "{:<14} {:<20} {}",
                    cmd.name,
                    cmd.aliases.join(", "),
                    cmd.description,
                );
            }
        }

        Some(cmd) => {
            println!("{}", cmd.description);
            println!();
            println!("Usage:");
            println!("  {}", cmd.usage());

            if let Some(subcommands) = cmd.subcommands {
                println!("\nSubcommands:");

                for sub in subcommands {
                    println!("  {:<12} {}", sub.name, sub.description);
                }
            }
        }
    }
}
