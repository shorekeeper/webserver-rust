pub trait InputValidator {
    fn is_valid(&self, input: &str) -> bool;
}

pub struct NonEmptyInputValidator;

impl InputValidator for NonEmptyInputValidator {
    fn is_valid(&self, input: &str) -> bool {
        !input.is_empty()
    }
}