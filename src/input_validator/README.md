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