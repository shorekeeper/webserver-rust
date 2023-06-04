use crate::input_validator::InputValidator;
use crate::form_config::{FormConfig, FormConfigImpl};
use crate::{log_info, log_warn, log_error};
use crate::log::{info, error, warn};
use crate::time::current_time;

use actix_web::{web, HttpResponse};
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use tera::{Tera};

#[allow(non_snake_case)]
pub async fn process_form(form: web::Form<std::collections::HashMap<String, String>>) -> HttpResponse {
    let now = current_time();
    let mut config = FormConfigImpl::new();

    for (key, value) in form.into_inner() {
        match config.input_validator().is_valid(&value) {
            false => {
                config.context().insert("error", &format!("{} cannot be empty", key));
                match (config.name().is_empty(), config.email().is_empty(), config.message_body().is_empty()) {
                    (true, true, true) => {
                        //if !*config.message_printed() {
                            config.context().insert("error", "smtp is not magic, type smth");
                            log_error!(&now, "User didn't entered anything for: {}", key);
                            // *config.message_printed() = true;
                        //}
                    },
                    _ => log_warn!(&now, "User didn't entered {}", key),
                }
                continue;
            }
            true => {
                config.context().insert(key.as_str(), &value);
                log_info!(&now, "User entered {} for {}", value, key);
                match key.as_str() {
                    "email" => config.set_email(value),
                    "name" => config.set_name(value),
                    "message" => config.set_message_body(value),
                    _ => (),
                }
            }
        }
    }
    

    let credentials = Credentials::new(config.smtp_user(), config.smtp_pass());
    let mailer = SmtpTransport::relay(&config.smtp_host())
        .unwrap()
        .credentials(credentials)
        .build();

    match (config.email_validator().is_valid(&config.email()), config.email_validator().is_valid(&config.smtp_user())) {
        (false, _) => config.context().insert("error", "Invalid email address"),
        (_, false) => config.context().insert("error", "Invalid SMTP username"),
        (true, true) => {
            let message = Message::builder()
                .from(config.smtp_user().parse().unwrap())
                .to(config.email().parse().unwrap())
                .subject("Form Submission")
                .body(format!(
                    "Thank you for your submission, {}!\n\nYour message:\n{}",
                    config.name(), config.message_body()
                ))
                .unwrap();
            // send the email message
            match mailer.send(&message) {
                Ok(_) => log_info!(&now, "Email sended: {}", config.email()),
                Err(e) => log_error!(&now, "Error sending email: {:?}", e),
            }
        }
    }
        

    let body = Tera::one_off(include_str!("templates/form.tera"), &config.context(), false).unwrap();
    HttpResponse::Ok().body(body)
}
