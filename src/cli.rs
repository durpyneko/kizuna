use crate::agent;

pub struct Command {
    pub name: &'static str,
    pub aliases: &'static [&'static str],
    pub usage: &'static str,
    pub description: &'static str,
}

pub const COMMANDS: &[Command] = &[
    Command {
        name: "agent",
        aliases: &["-a"],
        usage: "kizuna agent",
        description: "Start the Kizuna agent.",
    },
    Command {
        name: "connect",
        aliases: &["-c"],
        usage: "kizuna connect <host>",
        description: "Connect to a remote agent.",
    },
    Command {
        name: "update",
        aliases: &["-u"],
        usage: "kizuna update <url>",
        description: "Update Kizuna from a remote binary.",
    },
];

pub async fn handler(args: Vec<String>) -> std::io::Result<()> {
    let mut args_iter = args.iter();
    let (_exe, cmd, _arg) = (args_iter.next(), args_iter.next(), args_iter.next());

    // println!("Exe: {:?}\nCMD: {:?}\nArg: {:?}", exe, cmd, arg);

    match cmd.map(String::as_str) {
        Some("-a") | Some("agent") => {
            log::info!("Starting agent daemon...");
            agent::start().await?;
        }

        Some("-h") | Some("help") => {
            print_help();
        }

        Some(cmd) => {
            log::warn!("Unknown command: {cmd}");
            print_help();
        }

        None => {
            log::warn!("Please specify a command.");
            print_help();
        }
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
        println!("      Usage: {}", cmd.usage);
        println!();
    }
}
