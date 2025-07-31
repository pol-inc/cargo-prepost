use std::env;
use std::path::Path;
use std::process::Command;
use tempfile::tempdir_in;

pub fn execute_prepost(cargo: impl AsRef<Path>, path: impl AsRef<Path>) {
    let cargo = cargo.as_ref();
    let path = path.as_ref();

    let temp = match tempdir_in("./target") {
        Ok(v) => v,
        Err(e) => {
            log::error!("Failed to create temporary directory: {e}");
            std::process::exit(1);
        }
    };
    let src_dir = temp.path().join("src");
    std::fs::create_dir_all(&src_dir).unwrap();
    if let Err(e) = std::fs::copy(path, src_dir.join("main.rs")) {
        log::error!("Failed to copy {}: {}", path.display(), e);
        std::process::exit(1);
    }

    if let Err(e) = std::fs::copy("./Cargo.toml", temp.path().join("Cargo.toml")) {
        log::error!("Failed to write Cargo.toml: {e}");
    }

    let mut cmd = match Command::new(cargo)
        .current_dir(temp.path())
        .args(["run", "--release"])
        .spawn()
    {
        Ok(v) => v,
        Err(e) => {
            log::error!("Failed to spawn cargo: {e}");
            std::process::exit(1);
        }
    };
    match cmd.wait() {
        Ok(v) if v.success() => {
            log::info!("{} successfully executed", path.display());
        }
        _ => {
            log::error!("Failed to execute {}", path.display());
            std::process::exit(1);
        }
    }
}

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

    let pre_rs = subcommand.clone().map(|v| format!("pre{v}.rs"));
    let post_rs = subcommand.clone().map(|v| format!("post{v}.rs"));

    let pre_path = pre_rs.map(|v| prepost_dir.join(&v));
    let post_path = post_rs.map(|v| prepost_dir.join(&v));

    if let Some(pre_path) = pre_path
        && pre_path.is_file()
    {
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

    if let Some(post_path) = post_path
        && post_path.is_file()
    {
        log::info!("{} found", post_path.display());
        execute_prepost(&cargo, &post_path);
    }
}
