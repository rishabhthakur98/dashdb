use dotenvy::dotenv;
use std::env;
use crate::env_vars::EnvVariables;
use crate::dashdb_logging::init_tracing;
use tracing::info;

pub fn load_and_log_env() -> EnvVariables {
    dotenv().ok();

    let server_url = env::var("SERVER_URL").expect("SERVER_URL environment variable is not set");
    let log_level = env::var("LOG_LEVEL").expect("LOG_LEVEL environment variable is not set");
    let auth_token = env::var("AUTH_TOKEN").expect("AUTH_TOKEN environment variable is not set");

    init_tracing(&log_level);

    info!("SERVER_URL: {}", server_url);
    info!("LOG_LEVEL: {}", log_level);
    info!("AUTH_TOKEN: {}", auth_token);

    EnvVariables {
        server_url,
        log_level,
        auth_token,
    }
}