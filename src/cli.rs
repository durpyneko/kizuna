use crate::agent;
use std::process;

pub async fn handler(args: Vec<String>) -> std::io::Result<()> {
    let mut args_iter = args.iter();
    let (exe, cmd, arg) = (args_iter.next(), args_iter.next(), args_iter.next());

    println!("Exe: {:?}\nCMD: {:?}\nArg: {:?}", exe, cmd, arg);

    match cmd.map(String::as_str) {
        Some("-a") | Some("agent") => {
            log::info!("Starting agent daemon...");
            agent::start().await?;
        }

        Some(cmd) => {
            log::warn!("Unknown command: {cmd}");
            process::exit(1);
        }

        None => {
            log::warn!("Please specify a command.");
            process::exit(1);
        }
    }

    Ok(())
}
