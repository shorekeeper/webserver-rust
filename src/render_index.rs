use actix_web::HttpResponse;
use tera::{Context, Tera};

pub async fn index() -> HttpResponse {
    let mut context = Context::new(); // create a new Tera context
    context.insert("context", "Rust Index"); // insert data into the context
    context.insert("name", "User");
    // note: i would probably use a define here like static INDEX_PATH but then macros include_str!
    // should be replaced with "std::fs::read_to_string" which makes code runtime a bit so its bullshit
    // render the template with the context
    let body = Tera::one_off(include_str!("templates/index.tera"), &context, false)
        .expect("Failed to render template"); // handling error with except macros  
    HttpResponse::Ok().body(body) // return the rendered template as the response body
}