use std::env;
use dotenvy::dotenv;
use tracing::{info, error, instrument};

#[instrument(level = "trace")]
pub fn loaddotenv() -> String {
    dotenv().ok();
    
    match env::var("SERVER_URL") {
        Ok(server_url) => {
            info!("Using ip and port: {}", server_url);
            return server_url;
        }
        Err(e) => {
            error!("Error {:?}", e);
            info!("Using default ip and port: 127.0.0.1:8080");
            return String::from("127.0.0.1:8080");
        }
    }
}
