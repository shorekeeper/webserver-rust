use regex::Regex;

pub trait EmailValidator {
    fn is_valid(&self, email: &str) -> bool;
}

pub struct EmailRegexValidator {
    regex: Regex,
}

impl EmailRegexValidator {
    pub fn new() -> Self {
        let regex = Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)*$").unwrap();
        Self { regex }
    }
}

impl EmailValidator for EmailRegexValidator {
    fn is_valid(&self, email: &str) -> bool {
        self.regex.is_match(email)
    }
}
