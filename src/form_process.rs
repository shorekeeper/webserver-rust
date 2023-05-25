use actix_web::{web, HttpResponse};
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use tera::{Context, Tera};

// defining SMTP server credentials as static variables
static SMTP_USER: &str = "your_smtp_user";
static SMTP_PASS: &str = "your_smtp_pass";
static SMTP_HOST: &str = "your_smtp_host"; // WITHOUT SSL:// OR TLS://!!!

pub async fn process_form(form: web::Form<std::collections::HashMap<String, String>>) -> HttpResponse {
    // create a new Tera context
    let mut context = Context::new();
    context.insert("name", "User");
    context.insert("context", "Rust Form");

    let mut email = String::new();
    let mut name = String::new();
    let mut message_body = String::new();
    
    // iterate over the form data
    for (key, value) in form.into_inner() {
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
    let mailer = SmtpTransport::relay(SMTP_HOST)
        .unwrap()
        .credentials(credentials)
        .build();

    // validate the email address and SMTP username
    match (email.is_empty() || !email.contains('@'), SMTP_USER.is_empty() || !SMTP_USER.contains('@')) {
        (true, _) => context.insert("error", "Invalid email address"),
        (_, true) => context.insert("error", "Invalid SMTP username"),
        (false, false) => {
            // create an email message
            let message = Message::builder()
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
    // render the template with the context
    let body = Tera::one_off(include_str!("templates/form.tera"), &context, false).unwrap();
    // return the rendered template as the response body
    HttpResponse::Ok().body(body)
}
