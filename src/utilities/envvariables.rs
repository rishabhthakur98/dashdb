use std::env;
use dotenvy::dotenv;

pub fn loaddotenv() -> String {
    dotenv().ok();
    
    match env::var("SERVER_URL") {
        Ok(server_url) => {
            println!("Using ip and port: {}", server_url);
            return server_url;
        }
        Err(e) => {
            eprintln!("Error {:?}", e);
            println!("Using default ip and port: 127.0.0.1:8080");
            return String::from("0.0.0.0:8080");
        }
    }
}
