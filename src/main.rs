// core
mod cli;
mod common;
mod logger;
mod service;

// features
mod agent;

use owo_colors::OwoColorize;
use std::env;

#[tokio::main]
async fn main() {
    let level = if cfg!(debug_assertions) {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };

    logger::init(level);

    println!(
        "{}                       {}\n",
        common::colored_banner(),
        format!("{} {}", "version:", env!("CARGO_PKG_VERSION")).truecolor(25, 25, 25)
    );

    let args: Vec<String> = env::args().collect();

    let _ = cli::handler(args).await;
}
