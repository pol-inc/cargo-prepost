use clap::{Parser, Subcommand};
use std::env;
use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Setup {
        #[arg(long, value_name = "PATH")]
        path: Option<PathBuf>,
    },
}

pub fn main(args: impl Iterator<Item = impl Into<OsString> + Clone>) {
    let cli = Cli::parse_from(args);
    match cli.command {
        Some(Commands::Setup { path }) => {
            let path = match path {
                Some(v) => v,
                None => match env::home_dir() {
                    Some(v) => v.join(".cargo-prepost").join("bin"),
                    None => {
                        log::error!("Failed to get home directory");
                        std::process::exit(1);
                    }
                },
            };

            let new_cargo = path.join("cargo");
            if new_cargo.exists() {
                log::warn!("It seems that cargo-prepost is already setup");
            } else {
                let current_exe = match env::current_exe() {
                    Ok(v) => v,
                    Err(e) => {
                        log::error!("Failed to get current executable path: {e}");
                        std::process::exit(1);
                    }
                };
                if let Err(e) = std::fs::create_dir_all(&path) {
                    log::error!("Failed to create directory: {e}");
                    std::process::exit(1);
                }
                if let Err(e) = std::os::unix::fs::symlink(current_exe, new_cargo) {
                    log::error!("Failed to create cargo symlink: {e}");
                    std::process::exit(1);
                }
            }

            let mut new_path: std::collections::VecDeque<_> =
                match env::var("PATH").map(|v| env::split_paths(&v).collect()) {
                    Ok(v) => v,
                    _ => {
                        log::error!("Failed to get PATH");
                        std::process::exit(1);
                    }
                };
            new_path.push_front(path);
            let new_path = match env::join_paths(new_path) {
                Ok(v) => v,
                Err(e) => {
                    log::error!("Failed to get new PATH: {e}");
                    std::process::exit(1);
                }
            };
            println!("{}", new_path.display());
        }
        _ => {
            println!("cargo-prepost");
        }
    }
}
