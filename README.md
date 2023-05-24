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

## Credits

This code is based on the [Actix-Web Getting Started](https://actix.rs/docs/getting-started/) guide and uses the [Tera](https://tera.netlify.app/) templating engine.
