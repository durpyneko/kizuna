// core
mod cli;
mod common;
mod logger;

// features
mod agent;

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
        "{}                       version: {}\n",
        common::BANNER,
        env!("CARGO_PKG_VERSION")
    );

    let args: Vec<String> = env::args().collect();

    let _ = cli::handler(args).await;
}
