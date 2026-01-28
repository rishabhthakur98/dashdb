use tracing_appender::{non_blocking::WorkerGuard,rolling::{RollingFileAppender, Rotation}};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};


pub fn init_tracing() -> WorkerGuard {

    let file_appender = RollingFileAppender::new(
        Rotation::HOURLY, 
        "logs",           
        "app.log",       
    );

    let (non_blocking_writer, guard) = tracing_appender::non_blocking(file_appender);
    
    let console_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .pretty();


    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking_writer)
        .json();

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    Registry::default()
        .with(env_filter)
        .with(console_layer)
        .with(file_layer)
        .init();

    guard
    
    }
