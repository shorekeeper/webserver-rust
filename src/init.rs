use actix_web::{web, App, HttpServer};
use actix_web::middleware::{Logger, TrailingSlash::Trim};
use env_logger::{Env, Builder};

use crate::render_index::index;
use crate::form_process::process_form;
use crate::error_handler::{not_found, handle_error};

static SERVER_IP: &str = "127.0.0.1:8080"; // define the server IP address as a static variable

pub async fn create_server() -> std::io::Result<()> {
    println!("[INFO] 🚀 \x1b[33mTrying to run on: \x1b[31m{}\x1b[0m", SERVER_IP); // output server ip
    let server = match HttpServer::new(|| { // start the HTTP server
        App::new()  
            .wrap(Logger::default()) // logging
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_web::middleware::Compress::default())
            .wrap(actix_web::middleware::NormalizePath::new(Trim))
            .route("/", web::get().to(index)) // register routes and their handlers
            .route("/form", web::post().to(process_form))
            .app_data(web::Data::new(handle_error)) // register the error handler
            .default_service(
                actix_web::web::route().to(not_found)
            ) // default gateway for bad request -> like 404
    })

    .bind(SERVER_IP) {
        Ok(server) => { // if ok
            println!("[INFO] 📢 \x1B[1m\x1b[32mListening on: \x1b[31mhttp://{}\x1b[0m", SERVER_IP); // print the server IP address after the server starts
            println!("[INFO] ✅ \x1B[1m\x1B[4mOk bro now i'm gonna run ur site\x1b[0m");
            server
        }
        Err(e) => { // if NOT ok
            eprintln!("\x1b[31m\x1b[1mFailed to bind server to \x1B[4m{}:\x1b[33m{}\x1b[0m", SERVER_IP, e); // print the error
            return Err(e);
        }
    };

    server.run().await?; // run the server

    Ok(())
}


pub fn init_logger() { // initialize the logger
    Builder::from_env(Env::default().default_filter_or("info")).init();
}

// colors: 
// \x1b[32m - green
// \x1b[31m - red
// \x1B[4m - underline 
// \x1B[1m - bold
// \x1b[0m - reset