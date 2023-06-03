mod form_process;
mod render_index;
mod init;
mod error_handler;

use crate::init::{create_server, init_logger};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger(); // init logger????????
    let _server = create_server().await?; // running server lol somehow
    Ok(())
}