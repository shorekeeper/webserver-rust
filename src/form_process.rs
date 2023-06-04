use crate::input_validator::InputValidator;
use crate::form_config::{FormConfig, FormConfigImpl};
use crate::{log_info, log_warn, log_error}; // you should use all of this imports if u want to use log_<type>! macros
use crate::log::{info, error, warn}; //
use crate::time::current_time; //

use actix_web::{web, HttpResponse};
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials; 
use tera::{Tera};

#[allow(non_snake_case)]
pub async fn process_form(form: web::Form<std::collections::HashMap<String, String>>) -> HttpResponse {
    let now = current_time(); // get current time
    let mut config = FormConfigImpl::new(); // create a new instance of FormConfigImpl

    for (key, value) in form.into_inner() { // iterate over form data
        match config.input_validator().is_valid(&value) { // check if value is valid using input_validator
            false => { 
                config.context().insert("error", &format!("{} cannot be empty", key)); // insert error message into context
                match (config.name().is_empty(), config.email().is_empty(), config.message_body().is_empty()) {
                    (true, true, true) => {
                        // i could definitely use if statements here but i like match much (lol) more
                        config.context().insert("error", "smtp is not magic, type smth");
                        log_error!(&now, "User didn't entered anything for: {}", key);
                    },
                    _ => log_warn!(&now, "User didn't entered {}", key),
                }
                continue;
            }
            true => {
                config.context().insert(key.as_str(), &value);
                log_info!(&now, "User entered {} for {}", value, key);
                match key.as_str() { 
                    "email" => config.set_email(value), // set email in config
                    "name" => config.set_name(value), // set name in config
                    "message" => config.set_message_body(value), // set message_body in config
                    _ => (),
                }
            }
        }
    }

    let credentials = Credentials::new(config.smtp_user(), config.smtp_pass()); // create new Credentials instance with smtp_user and smtp_pass from config
    let mailer = SmtpTransport::relay(&config.smtp_host()) 
        .unwrap()
        .credentials(credentials)
        .build(); // create new SmtpTransport instance with smtp_host from config and credentials

    match (config.email_validator().is_valid(&config.email()), config.email_validator().is_valid(&config.smtp_user())) { 
        (false, _) => config.context().insert("error", "Invalid email address"), // insert error message into context if email is invalid
        (_, false) => config.context().insert("error", "Invalid SMTP username"), // insert error message into context if smtp_user is invalid
        (true, true) => {
            let message = Message::builder()
                .from(config.smtp_user().parse().unwrap())
                .to(config.email().parse().unwrap())
                .subject("Form Submission")
                .body(format!(
                    "Thank you for your submission, {}!\n\nYour message:\n{}",
                    config.name(), config.message_body()
                ))
                .unwrap(); // create new Message instance with smtp_user and email from config and formatted body text
            match mailer.send(&message) { 
                Ok(_) => log_info!(&now, "Email sended: {}", config.email()), 
                Err(e) => log_error!(&now, "Error sending email: {:?}", e),
            } 
        }
    }

    let body = Tera::one_off(include_str!("templates/form.tera"), &config.context(), false).unwrap(); 
    HttpResponse::Ok().body(body) 
}
