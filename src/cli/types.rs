use std::pin::Pin;

pub type Handler = fn(Vec<String>) -> CommandFuture;
pub type CommandFuture = Pin<Box<dyn Future<Output = std::io::Result<()>> + Send>>;

#[derive(Debug, Clone)]
pub struct Command {
    pub name: &'static str,
    pub aliases: &'static [&'static str],
    pub usage: &'static str,
    pub description: &'static str,
    pub subcommands: Option<&'static [Subcommand]>,
    pub handler: Handler,
}

#[derive(Debug, Clone)]
pub struct Subcommand {
    pub name: &'static str,
    pub description: &'static str,
}
