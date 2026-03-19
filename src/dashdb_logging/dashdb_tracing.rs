use std::str::FromStr;
use tracing::Level;

pub fn init_tracing(log_level: &str) {
    let level = Level::from_str(log_level).unwrap_or(Level::INFO);

    tracing_subscriber::fmt()
        .with_max_level(level)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();

}