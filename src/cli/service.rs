use super::Command;
use super::types::{CommandFuture, Subcommand};

pub const COMMAND: Command = Command {
    name: "service",
    aliases: &["-s", "--service"],
    usage: "kizuna service <install|uninstall|status>",
    description: "Manage the Kizuna system service.",
    subcommands: Some(SUBCOMMANDS),
    handler: run,
};

pub const SUBCOMMANDS: &[Subcommand] = &[
    Subcommand {
        name: "install",
        description: "Install the user service.",
    },
    Subcommand {
        name: "uninstall",
        description: "Remove the user service.",
    },
    Subcommand {
        name: "status",
        description: "Show the service status.",
    },
];

pub fn run(args: Vec<String>) -> CommandFuture {
    Box::pin(async move {
        let Some(action) = args.first() else {
            COMMAND.print_help();
            return Ok(());
        };

        match action.as_str() {
            "install" => crate::service::install(),
            "uninstall" => crate::service::uninstall(),
            other => {
                log::warn!("Unknown service subcommand: {}", other);
                COMMAND.print_help();
            }
        }

        Ok(())
    })
}
