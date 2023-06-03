mod form_process;
mod render_index;
mod init;
mod error_handler;
// mod uploads;

use dotenv::dotenv;
use std::{env, str::FromStr, net::SocketAddr};
use crate::init::{create_server, init_logger};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // load the .env file
    
    let server_ip = match env::var("SERVER_IP") {
        Ok(ip) => match ip.is_empty() {
            true => "NO_IP_CONFIGURED".to_string(),
            false => match ip.len() < 8 {
                true => "INVALID_IP_LENGTH".to_string(),
                false => match SocketAddr::from_str(&ip).is_ok() {
                    true => ip,
                    false => "INVALID_IP".to_string(),
                },
            },
        },
        Err(_) => "NO_IP_CONFIGURED".to_string(),
    };

    init_logger();
    let _server = create_server(&server_ip).await?; // pass the server_ip to the create_server function
    Ok(())
}