use std::env;

fn main() {
    simple_logger::init_with_env().expect("Failed to init logger");

    let args: Vec<_> = env::args().skip(1).collect();
    log::info!("args: {args:?}");
    let subcommand = args.iter().skip_while(|v| v.starts_with("-")).next();
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
