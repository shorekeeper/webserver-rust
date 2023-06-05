mod email_validator;
mod input_validator;
mod error_handler;
mod form_process;
mod render_index;
mod check_config;
mod form_config;
mod init;
// mod uploads;
use dotenv::dotenv;
use std::{env, str::FromStr, net::SocketAddr};
use crate::init::{create_server, init_logger};
use logger_rust::*;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // load the .env file
    init_logger(); // init logger
    let _ = check_config::check_config(); // call check_config function

    // writing debug info messages for checking env config data
    log_info!("SMTP_USER: {:?}", env::var("SMTP_USER"));
    log_info!("SMTP_HOST: {:?}", env::var("SMTP_HOST"));
    log_info!("SMTP_PASS: {:?}", env::var("SMTP_PASS"));
    log_info!("SERVER_IP: {:?}", env::var("SERVER_IP"));

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

    let _server = create_server(&server_ip).await?; // pass the server_ip to the create_server function
    Ok(())
}