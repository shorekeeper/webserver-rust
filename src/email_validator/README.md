# Validator Modules
This repository contains two validator modules: email_validator and input_validator.

## email_validator
- The `email_validator` module provides an implementation of the `EmailValidator` trait, which defines a method `is_valid` that takes an email address as input and returns a boolean indicating whether the email is valid.

- The `EmailRegexValidator` struct implements the `EmailValidator` trait using a regular expression to validate email addresses.

### Here's how can you use it:
```rust
use email_validator::{EmailValidator, EmailRegexValidator};

let validator = EmailRegexValidator::new();
let email = "example@example.com";
assert!(validator.is_valid(email));
```
> Note that you should also include configuration for correct usage ----=v

## Form config usage
The `form_config` module provides an implementation of the `FormConfig` trait, which defines methods for setting and retrieving form configuration values such as SMTP settings and user inputs. It also provides methods for accessing instances of the EmailValidator and InputValidator traits to validate user inputs.

The `FormConfigImpl` struct implements the `FormConfig` trait and uses instances of the `EmailRegexValidator` and `NonEmptyInputValidator` structs to validate email addresses and non-empty inputs, respectively.

These modules can be used together to validate user inputs in forms or other applications where user input is required. They provide a simple and easy-to-use interface for validating common types of inputs.