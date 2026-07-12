#[rustfmt::skip]
use {
    jiff::Zoned,
    std::io::Write,
    owo_colors::OwoColorize,
    log::{Level, LevelFilter},
    env_logger::{Builder, Env},
};

pub fn init(level: log::LevelFilter) {
    Builder::from_env(Env::default().default_filter_or(level.as_str()))
        .filter_level(LevelFilter::Warn) // deps
        .filter_module(env!("CARGO_PKG_NAME"), level) // "hide" all exept self
        .format(|buf, record| {
            let level = match record.level() {
                Level::Error => "[-]".red().bold().to_string(),
                Level::Warn => "[!]".yellow().bold().to_string(),
                Level::Info => "[*]".green().bold().to_string(),
                Level::Debug => "[+]".blue().bold().to_string(),
                Level::Trace => "[~]".magenta().bold().to_string(),
            };

            let now = Zoned::now();

            writeln!(
                buf,
                "{} {} {} {} {}",
                level,
                format!("[{}]", now.strftime("%H:%M:%S")).bright_black(),
                format!("[{}:{}]", record.target(), record.line().unwrap_or(0))
                    .cyan()
                    .bold(),
                ">⩊<".bright_black(),
                record.args(),
            )
        })
        .init();
}
