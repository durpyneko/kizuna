use super::Command;
use super::types::CommandFuture;

pub const COMMAND: Command = Command {
    name: "agent",
    aliases: &["-a", "--agent"],
    description: "Start the Kizuna agent.",
    subcommands: None,
    handler: run,
};

pub fn run(_args: Vec<String>) -> CommandFuture {
    Box::pin(async move {
        log::info!("Starting Agent...");
        crate::agent::start().await
    })
}
