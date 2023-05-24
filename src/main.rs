use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use tera::{Context, Tera};

async fn index() -> impl Responder {
    let mut context = Context::new();
    context.insert("context", "Rust Index");
    context.insert("name", "User");
    let body = Tera::one_off(include_str!("templates/index.tera"), &context, false).unwrap();
    HttpResponse::Ok().body(body)
}

async fn form(form: web::Form<std::collections::HashMap<String, String>>) -> impl Responder {
    let mut context = Context::new();
    context.insert("context", "Rust Form");
    for (key, value) in form.into_inner() {
        context.insert(key.as_str(), &value);
    }
    let body = Tera::one_off(include_str!("templates/form.tera"), &context, false).unwrap();
    HttpResponse::Ok().body(body)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/form", web::post().to(form))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
