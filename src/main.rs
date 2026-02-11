
use std::sync::Arc;
use dashmap::DashMap;
use tokio::net::TcpListener;
mod connection;
mod operations;
mod utilities;
use connection::handle_connection;
use utilities::envvariables::loaddotenv;

#[tokio::main]
async fn main() {
    let env_data = loaddotenv();


    let shared_map: Arc<DashMap<Vec<u8>, Vec<u8>>> = Arc::new(DashMap::new());



    match TcpListener::bind(env_data).await {
        
        
        Ok(listener) => {
            
            loop {


                 match listener.accept().await{
            Ok((stream, _addr)) =>{
                let db_clone = shared_map.clone();
                 tokio::spawn(async move {
            handle_connection(stream, db_clone).await;
        });
            } ,
            Err(e) => {
                eprintln!("Error {:?}", e);
            }
        
        }
       
        }
    
    }

        Err(e) => {
            eprintln!("Error {:?}", e);
        }
    }
}
