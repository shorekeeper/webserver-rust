use actix_web::{web, HttpResponse};
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use tera::{Context, Tera};

// defining SMTP server credentials as static variables
static SMTP_USER: &str = "your_smtp_user";
static SMTP_PASS: &str = "your_smtp_pass";
static SMTP_HOST: &str = "your_smtp_host" // WITHOUT SSL:// OR TLS://!!! ;

pub async fn process_form(form: web::Form<std::collections::HashMap<String, String>>) -> HttpResponse {
    // create a new Tera context
    let mut context = Context::new();
    // seeting up default "name" again otherwise we have panic:
    // thread 'actix-rt|system:0|arbiter:1' panicked at 'called `Result::unwrap()` on an `Err` 
    // value: Error { kind: Msg("Failed to render '__tera_one_off'"), source: 
    // Some(Error { kind: Msg("Variable `name` not found in context while rendering '__tera_one_off'"), source: None }) }', 
    // src/form_process.rs:84:84
    context.insert("name", "User");
    context.insert("context", "Rust Form");

    let mut email = String::new();
    let mut name = String::new();
    let mut message_body = String::new();
    
    // iterate over the form data
    for (key, value) in form.into_inner() {
        // iterate over the form data
        // check if the value is empty
        match value.is_empty() {
            true => {
                // insert an error message into the context
                context.insert("error", &format!("{} cannot be empty", key));
                // print debug message for user mistake
                match (name.is_empty(), email.is_empty(), message_body.is_empty()) {
                    // so, THIS SHIT doens't work and can be a pretty good example of shitty code
                    (true, true, true) => {
                        context.insert("error", "smtp is not magic, type smth");
                        println!("[INFO] User is bruh");
                    },
                    (true, _, _) => {
                        context.insert("error", "Username is empty");
                        println!("[WARN] User didn't enter a {}", key);
                    },
                    (_, true, _) => { 
                        context.insert("error", "Email is empty");
                        println!("[WARN] User didn't enter a {}", key);
                    },
                    (_, _, true) => { 
                        context.insert("error", "Message body is empty");
                        println!("[WARN] User didn't enter a {}", key);
                    },
                    (false, false, false) => (),
                }
                // skip to the next iteration
                continue;
            }
            false => {
                // insert the form data into the context
                context.insert(key.as_str(), &value);
                // print debug message for user actions
                println!("[INFO] User entered {} for {}", value, key);
                // extract email with name and message from the form data
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
