![Build Status](https://img.shields.io/github/actions/workflow/status/jknoptrix/rust-basic-webapp/.github/workflows/rust.yml?style=for-the-badge)
# Rust Actix-Web Example

This is a simple example of a Rust web application built using the Actix-Web framework. The application consists of two routes: an index page and a form submission page.
## Usage

1. Start the server:
   
   - `cargo run`
   
2. Open a web browser and navigate to `http://localhost:8080` to view the index page.
3. Click the "Submit" button to go to the form submission page.
4. Enter some data into the form and click "Submit" to see the submitted data on the page.

## Endpoints and code overview

- The `main()` (EP: `/` )  function sets up the Actix-Web server and defines the two routes.
- The `index()` (EP: `/templates/index` ) function handles requests to the root URL and renders the `index.tera` template using the Tera templating engine.
- The `form()` (EP: `/templates/form` ) function handles form submissions and renders the `form.tera` template with the submitted data.

# Form Processing

This Rust code processes a form submission using the Actix-web framework and sends an email using the Lettre crate.

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

## Notes

- This code contains some debug messages that can be removed or commented out.
- The Tera template used to render the response is located in the `templates/form.tera` file.

## Credits

This code is based on the [Actix-Web Getting Started](https://actix.rs/docs/getting-started/) guide and uses the [Tera](https://tera.netlify.app/) templating engine.
=======

