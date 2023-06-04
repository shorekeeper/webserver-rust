![Build Status](https://img.shields.io/github/actions/workflow/status/jknoptrix/rust-basic-webapp/.github/workflows/rust.yml?style=for-the-badge)
![Test](https://img.shields.io/badge/test-passing-green?style=for-the-badge)
# Rust Actix-Web Example

This is a simple example of a Rust web application built using the Actix-Web framework. The application consists of two routes: an index page and a form submission page.
## Usage

1. Setting up your SMTP and server credentials in `.env` file

2. Start the server:
   - `cargo build`
   - `cargo run`
> NOTE: I recommend to use a `cargo-watch` for checking code changes. It will increase compiling time.

3. Open a web browser and navigate to `<your domain>` to view the index page.

4. Click the "Submit" button to go to the form submission page.

5. Enter some data into the form and click "Submit" to see the submitted data on the page.

## Endpoints and code overview

- `main()` (EP: `/` )  function sets up the Actix-Web server and defines the two routes.
- `index()` (EP: `/templates/index` ) function handles requests to the root URL and renders the `index.tera` template using the Tera templating engine.
- `form()` (EP: `/templates/form` ) function handles form submissions and renders the `form.tera` template with the submitted data.

# Form Process Module

This module contains the `process_form` function which processes form data and sends an email using the SMTP protocol.

## Quick code description

The `process_form` function takes in a `web::Form` object containing a `HashMap` of form data. It then creates a new Tera context and inserts the name and context into it.

The SMTP server credentials are defined as static variables using the `env::var` function to retrieve their values from the `.env` file. These variables are `SMTP_USER`, `SMTP_PASS`, and `SMTP_HOST`.

The function then iterates over the form data and checks if any of the values are empty. If a value is empty, an error message is inserted into the context. If all values are non-empty, the function creates an email message using the `Message::builder` method and sends it using an SMTP transport.

The part of the code where you can change the sent message is where the email message is created using the `Message::builder` method. The part of the code where the SMTP credentials are declared is where the static variables `SMTP_USER`, `SMTP_PASS`, and `SMTP_HOST` are defined:
```rust
let SMTP_USER = env::var("SMTP_USER").expect("SMTP_USER must be set"); // get the SMTP_USER from the .env file
let SMTP_PASS = env::var("SMTP_PASS").expect("SMTP_PASS must be set"); // get the SMTP_PASS from the .env file
let SMTP_HOST = env::var("SMTP_HOST").expect("SMTP_HOST must be set"); // get the SMTP_HOST from the .env file | WITHOUT SSL:// OR TLS://!!!
```
These values will be readen from .env config file from program parent directory or root project dir. 
## Dependencies

This code uses the following dependencies:
- `actix-web`: for handling web requests and responses
- `lettre`: for sending emails via SMTP
- `tera`: for rendering templates

## Code usage

1. Set the SMTP server and server credentials by replacing the values of the `SMTP_SERVER`, `SMTP_USER`, and `SMTP_PASS`, `SERVER_IP` static variables with your own (host should not include ssl:// or tls://):
```env
SERVER_IP=127.0.0.1:8080
SMTP_USER=your_smtp_user
SMTP_PASS=your_smtp_pass
SMTP_HOST=your_smtp_host
```
2. The `process_form` function takes in a `web::Form` object containing form data as a key-value pair and returns an `HttpResponse`.
3. The function checks if the form data is empty and inserts an error message into the Tera context if it is.
4. The function iterates over the form data and checks if any value is empty. If a value is empty, an error message is inserted into the context and the iteration continues.
5. If all values are present, the function creates an email message using the Lettre crate and sends it using the provided SMTP server credentials.

# Logging Macros
Project have logging macros for different log levels: `error`, `warn`, and `info`. These macros allow you to easily log messages with the current time and appropriate formatting for each log level.

## Usage
To use these macros, you need to declare the variable `now` using `let now = current_time();`. This is necessary because the macros require a reference to the current time to properly format the log messages.

You also need to add the following crate imports: `use crate::log::{<log type>}; use crate::{<method>};`. These imports are necessary to use the logging functions and the `current_time` function.

To log a message, use the appropriate macro for the desired log level. For example, to log an error message, use the `log_error!` macro like this: `log_error!(&now, "<message>");`. Note that you must pass a reference to `now` as the first argument to the macro.

### Example:
```rust
use crate::log_info; // macro imports
use crate::log::info; // method imports
use crate::time::current_time; // time import

fn main() {
    let now = current_time();
    log_info!(&now, "This is an informational message");
}
```
> The `log_info!` macro is defined using the macro_rules! macro. It takes two arguments: a reference to the current time `($now:expr)` and a format string with any additional arguments `($($arg:tt)*)`. The macro expands to a call to the info function with the current time and the formatted message as arguments.

The info function takes two arguments: 
- A reference to the current time `(now: &str)`;
- The message to log `(message: &str)`. 
It uses the colored crate to format the log message with appropriate colors and styles for an informational message. The formatted message is then printed to standard output using the println! macro.

## Time Module
The `current_time` function is defined in a separate module because it uses the `chrono` crate to get the current time. This function cannot be defined in the same module as the macros because Rust.

## Error handler
The `error_handler` module contains all possible (for my opinion) web errors, but if you want to modify it, here's how can you do this:
```rust
let response = match error.as_response_error().error_response().status() {
    StatusCode::<STATUS CODE> => {
        HttpResponse::BadRequest().json(json!({
             "error": "<error>",
             "message": <error_message>
        }))
    },
   // and some code below
```
> Note that all of the `StatusCode::` errors should be declared in `let response = ...` construction.

# Form Configuration Module

This module provides a global form configuration and handler. It allows you to set and retrieve form data such as email, name, and message body. It also provides methods for validating email and non-empty input.

## Usage

To use this module, you need to create an instance of `FormConfigImpl`:

```rust
use crate::form_config::{FormConfig, FormConfigImpl};
// ..
let mut form_config = FormConfigImpl::new();
```
You can retrieve the form data using the corresponding getter methods:
```rust
let email = form_config.email();
let name = form_config.name();
let message_body = form_config.message_body();
```
You can also access the SMTP configuration by calling the `smtp_user, smtp_pass, and smtp_host` methods.

The module also provides an `input_validator` method that returns a reference to a NonEmptyInputValidator instance, and an email_validator method that returns a reference to an `EmailValidator` instance.

## Adding Custom Variables
To add custom variables to the form configuration, you can modify the `FormConfigImpl` struct and add new fields. You can then implement the corresponding setter and getter methods in the `FormConfig` trait and provide an implementation in the `FormConfigImpl` struct.

For example, to add a phone number field, you can do the following:

```rust
pub trait FormConfig {
    // ...
    fn set_phone(&mut self, phone: String);
    fn phone(&self) -> String;
}

pub struct FormConfigImpl {
    // ...
    phone: String,
}

impl FormConfig for FormConfigImpl {
    // ...
    fn set_phone(&mut self, phone: String) {
        self.phone = phone;
    }

    fn phone(&self) -> String {
        self.phone.clone()
    }
}
```
### Note that adding config variables like SMS-service requires adding them as env. variables in .env file and you should change the `const` of default config to your own:
```rust
const DEFAULT_CONFIG: &str = 
r#"# !! LOCALHOST: 127.0.0.1:8080
# !! IP SHOULD BE DECLARED IN FORMAT IP:PORT
SERVER_IP=your_ip
SMTP_USER=your_smtp_user
SMTP_PASS=your_smtp_pass
SMTP_HOST=your_smtp_host
DATABASE_URL=postgres://user:password@host/database"#;
#some of your datas
```
## Context

The `FormConfigImpl` struct also contains a `context` field that holds a `Context` instance from the `tera` crate. You can access and modify this context using the `context` method:

```rust
let mut context = form_config.context();
context.insert("key", "value");
```
> This allows you to add custom data to the context that can be used when rendering templates.

## Validation
The module provides two validators: an `email validator` and a non-empty `input validator`. You can access these validators using the `email_validator` and `input_validator` methods:
```rust
let email_validator = form_config.email_validator();
let input_validator = form_config.input_validator();
```
You can also then use these validators to validate form data:
```rust
let is_email_valid = email_validator.is_valid(&form_config.email());
let is_name_valid = input_validator.is_valid(&form_config.name());
```

## Notes

- This code contains some debug messages that can be removed or commented out.
- The Tera template used to render the response is located in the `templates/form.tera` file.
- I have some TODO here

## TODO:

Later I want to add an:
- authorization system
- the ability to upload files
- cookies;
But this requires a lot of work. For example, I already have a files upload:
```rust
pub async fn upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // iterate over the multipart fields and save each one to a file
    while let Ok(Some(field)) = payload.try_next().await {
        save_field(field).await?;
    }

    Ok(HttpResponse::Ok().into())
}
pub fn init<T>(app: App<T>) -> App<T>
where
    T: ServiceFactory<ServiceRequest, Config = (), Response = actix_web::dev::ServiceResponse, Error = actix_web::Error, InitError = ()>,
{
    app.route("/upload", web::post().to(upload))
}
```

But for now it gives a bunch of errors that I’m too lazy to solve, and the problem is traits. I’m not strong in Rust and I’m still a beginner, so it’s a bit difficult for me. There is also an option to connect to the database and serialize data via JSON now:
```rust
async fn get_data(query: web::Query<Query>) -> HttpResponse {
    // Retrieve data from database using the provided id
    let data = get_data_from_database(query.id).await;

    // Return data as a JSON response
    HttpResponse::Ok().json(data)
}
async fn get_data_from_database(id: i32) -> serde_json::Value {
    // Example data retrieval from database
    json!({
        "id": id,
        "name": "Example Data",
        "value": 42
    })
}
```

> I think I will definitely add this later, but not now.

## Credits

This code is based on the [Actix-Web Getting Started](https://actix.rs/docs/getting-started/) guide and uses the [Tera](https://tera.netlify.app/) templating engine.
=======

