use crate::agent;

pub struct Command {
    pub name: &'static str,
    pub aliases: &'static [&'static str],
    pub usage: &'static str,
    pub description: &'static str,
}

pub const COMMANDS: &[Command] = &[Command {
    name: "agent",
    aliases: &["-a"],
    usage: "kizuna agent",
    description: "Start the Kizuna agent.",
}];

impl Command {
    pub fn matches(&self, input: &str) -> bool {
        self.name == input || self.aliases.contains(&input)
    }
}

pub fn find_command(input: &str) -> Option<&'static Command> {
    COMMANDS.iter().find(|cmd| cmd.matches(input))
}

pub async fn handler(args: Vec<String>) -> std::io::Result<()> {
    let mut args_iter = args.iter();
    let (_exe, cmd, _arg) = (args_iter.next(), args_iter.next(), args_iter.next());

    let Some(cmd) = cmd else {
        log::warn!("Please specify a command.");
        print_help();
        return Ok(());
    };

    let Some(command) = find_command(cmd) else {
        log::warn!("Unknown command: {}", cmd);
        print_help();
        return Ok(());
    };

    match command.name {
        "agent" => {
            log::info!("Starting agent daemon...");
            agent::start().await?;
        }

        _ => unreachable!(),
    }

    Ok(())
}

pub fn print_help() {
    // println!("{}", crate::common::BANNER);
    println!("Usage:");
    println!("  kizuna <command> [options]\n");

    println!("Commands:");

    for cmd in COMMANDS {
        println!(
            "  {:<12} {:<18} {}",
            cmd.name,
            cmd.aliases.join(", "),
            cmd.description
        );
        println!("\tUsage: {}", cmd.usage);
        println!();
    }
}
