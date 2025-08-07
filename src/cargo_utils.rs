use cargo_metadata::MetadataCommand;
use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub fn execute_prepost(cargo: impl AsRef<Path>, path: impl AsRef<Path>) {
    let cargo = cargo.as_ref();
    let path = path.as_ref();

    let target_dir = match MetadataCommand::new().cargo_path(cargo).exec() {
        Ok(v) => v.target_directory.into_std_path_buf(),
        Err(e) => {
            log::warn!("Failed to get cargo metadata: {e}; use default values");
            // TODO: Windows support
            std::fs::canonicalize("./target").unwrap_or(PathBuf::from("./target"))
        }
    };
    let target_dir = target_dir.join("prepost");
    if let Err(e) = std::fs::create_dir_all(&target_dir) {
        log::error!("Failed to create target directory: {e}");
        std::process::exit(1);
    }

    let mut cmd = if path
        .extension()
        .map(|v| v.to_str() == Some("rs"))
        .unwrap_or(false)
    {
        let target_bin_name = path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .strip_suffix(".rs")
            .unwrap();
        let target_file = target_dir.join(target_bin_name);
        let cargo_toml_path = path.with_file_name("Cargo.toml");
        if cargo_toml_path.is_file() {
            let mut cmd = Command::new(cargo);
            cmd.args(["run", "--bin", target_bin_name, "--manifest-path"])
                .arg(&cargo_toml_path)
                .stderr(Stdio::null());
            cmd
        } else {
            let mut child = match Command::new("rustc")
                .arg("-o")
                .arg(&target_file)
                .arg(path)
                .spawn()
            {
                Ok(v) => v,
                Err(e) => {
                    log::error!("Failed to spawn rustc: {e}");
                    std::process::exit(1);
                }
            };
            child.wait().unwrap();
            Command::new(&target_file)
        }
    } else {
        Command::new(path)
    };

    let mut child = match cmd.spawn() {
        Ok(v) => v,
        Err(e) => {
            log::error!("Failed to spawn specified executable: {e}");
            std::process::exit(1);
        }
    };
    match child.wait() {
        Ok(v) if v.success() => {
            log::info!("{} successfully executed", path.display());
        }
        Ok(v) => {
            log::warn!("{} executed; but returns {:?}", path.display(), v.code());
        }
        Err(e) => {
            log::error!("Failed to execute {}: {e}", path.display());
            std::process::exit(1);
        }
    }
}

/// entry point of hooked cargo
pub fn main(args: impl Iterator<Item = impl ToString>) {
    let args: Vec<_> = args.map(|v| v.to_string()).collect();
    let subcommand = args.first().cloned();

    let path = match env::var("PATH") {
        Ok(v) => v,
        _ => {
            log::error!("Failed to get PATH");
            std::process::exit(1);
        }
    };
    let paths = env::split_paths(&path);
    let mut cargo = None;
    for path in paths {
        let path = path.join("cargo");
        if path.is_file()
            && env::current_exe().expect("Failed to fetch current executable path") != path
        {
            log::info!("Find default cargo: {}", path.display());
            cargo = Some(path);
            break;
        }
    }
    let cargo = match cargo {
        Some(v) => v,
        None => {
            log::error!("Failed to find default cargo");
            std::process::exit(1);
        }
    };

    let cwd = match env::current_dir() {
        Ok(v) => v,
        Err(e) => {
            log::error!("Failed to get current working directory: {e}");
            std::process::exit(1);
        }
    };
    let prepost_dir = cwd.join("prepost");

    let pre_executable = subcommand.clone().map(|v| format!("pre{v}"));
    let post_executable = subcommand.clone().map(|v| format!("post{v}"));
    let pre_rs = subcommand.clone().map(|v| format!("pre{v}.rs"));
    let post_rs = subcommand.clone().map(|v| format!("post{v}.rs"));

    let pre_path = match (
        pre_executable.map(|v| prepost_dir.join(v)),
        pre_rs.map(|v| prepost_dir.join(&v)),
    ) {
        (Some(v), _) if v.is_file() => Some(v),
        (_, Some(v)) if v.is_file() => Some(v),
        _ => None,
    };
    let post_path = match (
        post_executable.map(|v| prepost_dir.join(v)),
        post_rs.map(|v| prepost_dir.join(&v)),
    ) {
        (Some(v), _) if v.is_file() => Some(v),
        (_, Some(v)) if v.is_file() => Some(v),
        _ => None,
    };

    if let Some(pre_path) = pre_path {
        log::info!("{} found", pre_path.display());
        execute_prepost(&cargo, &pre_path);
    }

    let mut cargo_cmd = Command::new(&cargo);
    let mut child = match cargo_cmd.args(&args).spawn() {
        Ok(v) => {
            log::info!("cargo successfully spawned");
            v
        }
        Err(e) => {
            log::error!("Failed to spawn cargo: {e}");
            std::process::exit(1);
        }
    };
    match child.wait() {
        Ok(v) if v.success() => {
            log::info!("cargo exited");
        }
        Ok(v) => {
            log::warn!("cargo exited with error status");
            if let Some(code) = v.code() {
                std::process::exit(code);
            } else {
                std::process::exit(1);
            }
        }
        Err(e) => {
            log::error!("Failed to wait child process: {e}");
            std::process::exit(1);
        }
    }

    if let Some(post_path) = post_path {
        log::info!("{} found", post_path.display());
        execute_prepost(&cargo, &post_path);
    }
}
