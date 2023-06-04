use crate::email_validator::{EmailRegexValidator, EmailValidator};
use crate::input_validator::NonEmptyInputValidator;
use std::env;
use tera::Context;

pub trait FormConfig {
    fn set_email(&mut self, email: String);
    fn set_name(&mut self, name: String);
    fn set_message_body(&mut self, message_body: String);
    fn smtp_user(&self) -> String;
    fn smtp_pass(&self) -> String;
    fn smtp_host(&self) -> String;
    fn input_validator(&self) -> &NonEmptyInputValidator;
    fn message_printed(&mut self) -> &mut bool;
    fn email(&self) -> String;
    fn name(&self) -> String;
    fn message_body(&self) -> String;
    fn context(&mut self) -> &mut Context;
    fn email_validator(&self) -> &dyn EmailValidator;
}

pub struct FormConfigImpl {
    smtp_user: String,
    smtp_pass: String,
    smtp_host: String,
    email_validator: EmailRegexValidator,
    input_validator: NonEmptyInputValidator,
    message_printed: bool,
    email: String,
    name: String,
    message_body: String,
    context: Context,
}

impl FormConfigImpl {
    pub fn new() -> Self {
        let smtp_user = env::var("SMTP_USER").expect("SMTP_USER must be set");
        let smtp_pass = env::var("SMTP_PASS").expect("SMTP_PASS must be set");
        let smtp_host = env::var("SMTP_HOST").expect("SMTP_HOST must be set");
        let email_validator = EmailRegexValidator::new();
        let input_validator = NonEmptyInputValidator;
        let mut context = Context::new();
        context.insert("name", "User");
        context.insert("context", "Rust Form");

        Self {
            smtp_user,
            smtp_pass,
            smtp_host,
            email_validator,
            input_validator,
            message_printed: false,
            email: String::new(),
            name: String::new(),
            message_body: String::new(),
            context,
        }
    }
}

impl FormConfig for FormConfigImpl {
    fn smtp_user(&self) -> String {
        self.smtp_user.clone()
    }

    fn smtp_pass(&self) -> String {
        self.smtp_pass.clone()
    }

    fn smtp_host(&self) -> String {
        self.smtp_host.clone()
    }

    fn email_validator(&self) -> &dyn EmailValidator {
        &self.email_validator
    }

    fn input_validator(&self) -> &NonEmptyInputValidator {
        &self.input_validator
    }

    fn message_printed(&mut self) -> &mut bool {
        &mut self.message_printed
    }

    fn email(&self) -> String {
        self.email.clone()
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn message_body(&self) -> String {
        self.message_body.clone()
    }

    fn context(&mut self) -> &mut Context {
        &mut self.context
    }

    fn set_email(&mut self, email: String) {
        self.email = email;
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn set_message_body(&mut self, message_body: String) {
        self.message_body = message_body;
    }
}
