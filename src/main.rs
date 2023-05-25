use actix_web::{web, App, HttpServer};
use actix_web::HttpResponse;
use tera::{Context, Tera};

mod form_process;

// define the server IP address as a static variable
static SERVER_IP: &str = "127.0.0.1:8080";

// handler function for the index route
async fn index() -> HttpResponse {
    // create a new Tera context
    let mut context = Context::new();
    // insert data into the context
    context.insert("context", "Rust Index");
    context.insert("name", "User");

    // render the template with the context
    // note: i would probably use a define here like static INDEX_PATH but then macros include_str!
    // should be replaced with "std::fs::read_to_string" which makes code runtime a bit so its bullshit
    let body = Tera::one_off(include_str!("templates/index.tera"), &context, false)
        .expect("Failed to render template"); // handling error with except macros
    // return the rendered template as the response body
    HttpResponse::Ok().body(body)

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // \x1b[32m - green
    // \x1b[31m - blue 
    // \x1b[0m - reset
    println!("[INFO] \x1b[33mTrying to run on: \x1b[31m{}\x1b[0m", SERVER_IP); // output server ip
    // start the HTTP server
    let server = match HttpServer::new(|| {
        App::new()
            // register routes and their handlers
            .route("/", web::get().to(index))
            .route("/form", web::post().to(form_process::process_form))
    })
    .bind(SERVER_IP) {
        // if ok
        Ok(server) => {
            // print the server IP address after the server starts
            println!("[INFO] 📢 \x1b[32mListening on: \x1b[31m{}\x1b[0m", SERVER_IP);
            println!("[INFO] Ok bro now i'm gonna run ur site");
            server
        }
        // if NOT ok
        Err(e) => {
            // print the error
            eprintln!("Failed to bind server to {}: {}", SERVER_IP, e);
            return Err(e);
        }
    };

    // run the server
    server.run().await?;

    Ok(())
}
