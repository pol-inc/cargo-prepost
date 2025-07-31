use std::env;

fn main() {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Off)
        .env()
        .init()
        .expect("Failed to init logger");

    let args: Vec<_> = env::args().skip(1).collect();
    log::info!("args: {args:?}");
    let subcommand = args.iter().find(|v| !v.starts_with("-"));
    log::info!("subcommand: {subcommand:?}");

    match subcommand.map(|v| v.as_str()) {
        Some("prepost") => {
            cargo_prepost::prepost_utils::main(args.iter());
        }
        _ => {
            cargo_prepost::cargo_utils::main(args.iter());
        }
    }
}
