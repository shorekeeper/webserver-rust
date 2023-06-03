use actix_web::{web, HttpResponse};
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use tera::{Context, Tera};
use std::env;

#[allow(non_snake_case)]
pub async fn process_form(form: web::Form<std::collections::HashMap<String, String>>) -> HttpResponse {
     
    let mut context = Context::new(); // create a new Tera context
    context.insert("name", "User");
    context.insert("context", "Rust Form");

    // defining SMTP server credentials as static variables
    let SMTP_USER = env::var("SMTP_USER").expect("[ERROR] SMTP_USER must be set"); // get the SMTP_USER from the .env file
    let SMTP_PASS = env::var("SMTP_PASS").expect("[ERROR] SMTP_PASS must be set"); // get the SMTP_PASS from the .env file
    let SMTP_HOST = env::var("SMTP_HOST").expect("[ERROR] SMTP_HOST must be set"); // get the SMTP_HOST from the .env file | WITHOUT SSL:// OR TLS://!!!

    let mut email = String::new();
    let mut name = String::new();
    let mut message_body = String::new();
    
    for (key, value) in form.into_inner() { // iterate over the form data
        match value.is_empty() {
            true => {
                context.insert("error", &format!("{} cannot be empty", key));
                match (name.is_empty(), email.is_empty(), message_body.is_empty()) {
                    (true, true, true) => {
                        context.insert("error", "smtp is not magic, type smth");
                        println!("[INFO] User is bruh");
                    },
                    _ => println!("[INFO] User didn't entered {}", key),
                }
                continue;
            }
            false => {
                context.insert(key.as_str(), &value);
                println!("[INFO] User entered {} for {}", value, key);
                match key.as_str() {
                    "email" => email = value,
                    "name" => name = value,
                    "message" => message_body = value,
                    _ => (),
                }
            }
        }
    }
    // creating let with SMTP credentials
    let credentials = Credentials::new(SMTP_USER.to_string(), SMTP_PASS.to_string());

    // creating let for an SMTP transport
    // i was trying to make relay(SMTP_SERVER) but it looks like that relay doesn't like this idk why
    let mailer = SmtpTransport::relay(&SMTP_HOST)
        .unwrap()
        .credentials(credentials)
        .build();

    match (email.is_empty() || !email.contains('@'), SMTP_USER.is_empty() || !SMTP_USER.contains('@')) { // validate the email address and SMTP username
        (true, _) => context.insert("error", "Invalid email address"),
        (_, true) => context.insert("error", "Invalid SMTP username"),
        (false, false) => {
            let message = Message::builder() // create an email message
                .from(SMTP_USER.parse().unwrap())
                .to(email.parse().unwrap())
                .subject("Form Submission")
                .body(format!(
                    "Thank you for your submission, {}!\n\nYour message:\n{}",
                    name, message_body
                ))
                .unwrap();

            // send the email message
            match mailer.send(&message) {
                Ok(_) => println!("[INFO] Email sended: {}", email),
                Err(e) => eprintln!("[ERROR] Error sending email: {:?}", e),
            }
        }
    }
    let body = Tera::one_off(include_str!("templates/form.tera"), &context, false).unwrap(); // render the template with the context
    HttpResponse::Ok().body(body) // return the rendered template as the response body
}
