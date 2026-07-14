use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

const SERVICE_NAME: &str = "kizuna.service";

#[derive(Clone, Copy)]
enum Scope {
    User,
    System,
}

impl Scope {
    fn label(self) -> &'static str {
        match self {
            Scope::User => "user",
            Scope::System => "system",
        }
    }

    fn wanted_by(self) -> &'static str {
        match self {
            Scope::User => "default.target",
            Scope::System => "multi-user.target",
        }
    }

    fn unit_path(self) -> io::Result<PathBuf> {
        match self {
            Scope::System => Ok(PathBuf::from("/etc/systemd/system").join(SERVICE_NAME)),
            Scope::User => {
                let config = std::env::var_os("XDG_CONFIG_HOME")
                    .map(PathBuf::from)
                    .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))
                    .ok_or_else(|| {
                        io::Error::new(
                            io::ErrorKind::NotFound,
                            "neither XDG_CONFIG_HOME nor HOME is set",
                        )
                    })?;
                Ok(config.join("systemd/user").join(SERVICE_NAME))
            }
        }
    }

    fn app_home(self) -> io::Result<PathBuf> {
        match self {
            Scope::System => Ok(PathBuf::from("/root/.kizuna")),
            Scope::User => std::env::var_os("HOME")
                .map(|h| PathBuf::from(h).join(".kizuna"))
                .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "HOME is not set")),
        }
    }
}

fn systemd_available() -> bool {
    std::path::Path::new("/run/systemd/system").is_dir()
}

pub fn install() {
    if !systemd_available() {
        log::error!("systemd was not detected (no /run/systemd/system); cannot install a unit.");
        return;
    }

    let Some(scope) = prompt_scope() else {
        log::warn!("Installation cancelled.");
        return;
    };

    match install_unit(scope) {
        Ok(installed) => report(scope, installed),

        Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
            log::error!(
                "Permission denied — a system service needs root. Try: sudo kizuna service install"
            );
        }

        Err(e) => log::error!("Failed to install service: {}", e),
    }
}

fn prompt_scope() -> Option<Scope> {
    loop {
        print!("Install Kizuna as a [u]ser or [s]ystem service? [u/s/q] ");
        io::stdout().flush().ok()?;

        let mut input = String::new();
        if io::stdin().read_line(&mut input).ok()? == 0 {
            return None; // EOF
        }

        match input.trim().to_lowercase().as_str() {
            "u" | "user" => return Some(Scope::User),
            "s" | "system" => return Some(Scope::System),
            "q" | "quit" => return None,
            other => println!("  please enter 'u', 's', or 'q' (got '{other}')"),
        }
    }
}

struct Installed {
    bin_path: PathBuf,
    started: bool,
}

fn install_unit(scope: Scope) -> io::Result<Installed> {
    let app_home = scope.app_home()?;
    let bin_path = app_home.join("kizuna");
    std::fs::create_dir_all(&app_home)?;

    let unit = format!(
        "[Unit]\n\
         Description=Kizuna Remote Agent\n\
         Documentation=https://github.com/durpyneko/kizuna\n\
         After=network-online.target\n\
         Wants=network-online.target\n\
         \n\
         [Service]\n\
         Type=simple\n\
         WorkingDirectory=%h/.kizuna\n\
         ExecStart=%h/.kizuna/kizuna agent\n\
         Restart=always\n\
         RestartSec=1\n\
         \n\
         # Graceful shutdown\n\
         TimeoutStopSec=10\n\
         KillSignal=SIGTERM\n\
         \n\
         # Logging\n\
         StandardOutput=journal\n\
         StandardError=journal\n\
         \n\
         [Install]\n\
         WantedBy={wanted}\n",
        wanted = scope.wanted_by(),
    );

    let unit_path = scope.unit_path()?;
    if let Some(dir) = unit_path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    std::fs::write(&unit_path, unit)?;
    log::info!("Wrote unit file to {}", unit_path.display());

    systemctl(scope, &["daemon-reload"])?;
    systemctl(scope, &["enable", SERVICE_NAME])?;

    let started = bin_path.exists();
    if started {
        systemctl(scope, &["start", SERVICE_NAME])?;
    }

    Ok(Installed { bin_path, started })
}

fn report(scope: Scope, installed: Installed) {
    if installed.started {
        log::info!(
            "Installed and started the Kizuna {} service.",
            scope.label()
        );
    } else {
        log::info!(
            "Installed the Kizuna {} service (enabled, not yet started).",
            scope.label(),
        );

        let start_cmd = match scope {
            Scope::User => "systemctl --user start kizuna.service",
            Scope::System => "sudo systemctl start kizuna.service",
        };
        log::warn!(
            "Place the agent binary at {}, then run: {}",
            installed.bin_path.display(),
            start_cmd,
        );
    }

    if let Scope::User = scope {
        log::info!("Tip: `loginctl enable-linger $USER` keeps it running after logout.");
    }
}

fn systemctl(scope: Scope, args: &[&str]) -> io::Result<()> {
    let mut cmd = Command::new("systemctl");
    if let Scope::User = scope {
        cmd.arg("--user");
    }
    cmd.args(args);

    let status = cmd.status()?;
    if !status.success() {
        return Err(io::Error::other(format!(
            "`systemctl {}` failed with {}",
            args.join(" "),
            status,
        )));
    }

    Ok(())
}

pub fn uninstall() {
    // systemd
}
