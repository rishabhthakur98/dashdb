use std::env;
use crate::utilities::EnvVariables;

pub fn load_and_log_env() -> EnvVariables {
    let server_url = env::var("SERVER_URL").expect("SERVER_URL environment variable is not set");
    let log_level = env::var("LOG_LEVEL").expect("LOG_LEVEL environment variable is not set");
    let auth_token = env::var("AUTH_TOKEN").expect("AUTH_TOKEN environment variable is not set");

    println!("SERVER_URL: {}", server_url);
    println!("LOG_LEVEL: {}", log_level);

    EnvVariables {
        server_url,
        log_level,
        auth_token,
    }
}