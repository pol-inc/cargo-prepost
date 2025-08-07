fn main() {
    simple_logger::init_with_level(log::Level::max()).unwrap();

    log::info!("postrun with dependency (log crate)")
}
