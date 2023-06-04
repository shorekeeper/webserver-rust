# Validator Modules
## input_validator
- The `input_validator` module provides an implementation of the `InputValidator` trait, which defines a method `is_valid` that takes an input string as input and returns a boolean indicating whether the input is valid.

- The `NonEmptyInputValidator` struct implements the `InputValidator` trait by checking if the input string is not empty.

### Here's how can you use it:
```rust
use input_validator::{InputValidator, NonEmptyInputValidator};

let validator = NonEmptyInputValidator;
let input = "some input";
assert!(validator.is_valid(input));
```
> Note that you should also include configuration for correct usage ----=v

## Form config usage
The `form_config` module provides an implementation of the `FormConfig` trait, which defines methods for setting and retrieving form configuration values such as SMTP settings and user inputs. It also provides methods for accessing instances of the EmailValidator and InputValidator traits to validate user inputs.

The `FormConfigImpl` struct implements the `FormConfig` trait and uses instances of the `EmailRegexValidator` and `NonEmptyInputValidator` structs to validate email addresses and non-empty inputs, respectively.

These modules can be used together to validate user inputs in forms or other applications where user input is required. They provide a simple and easy-to-use interface for validating common types of inputs.