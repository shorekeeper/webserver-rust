use serde_json::json;
use actix_web::{
    error, 
    HttpResponse, 
    dev::ServiceRequest,
    http::StatusCode};

pub async fn not_found() -> HttpResponse { // error handler for unregistered requests
    HttpResponse::NotFound().body("Sorry, the requested resource could not be found.")
}

pub fn handle_error(req: ServiceRequest, error: error::Error) -> actix_web::Result<HttpResponse> {
    let error_message = format!("{}", error);
    let method = req.method().to_string();
    let path = req.path().to_string();
    let headers = req.headers();

    // log information about the request
    println!("Error occurred while processing request: {} {}", method, path);
    println!("Headers: {:?}", headers);

    let response = match error.as_response_error().error_response().status() {
        StatusCode::BAD_REQUEST => {
            HttpResponse::BadRequest().json(json!({
                "error": "bad_request",
                "message": error_message
            }))
        },
        StatusCode::UNAUTHORIZED => {
            HttpResponse::Unauthorized().json(json!({
                "error": "unauthorized",
                "message": error_message
            }))
        },
        StatusCode::FORBIDDEN => {
            HttpResponse::Forbidden().json(json!({
                "error": "unauthorized",
                "message": error_message
            }))
        },
        StatusCode::NOT_FOUND => {
            HttpResponse::NotFound().json(json!({
                "error": "not_found",
                "message": error_message
            }))
        },
        StatusCode::METHOD_NOT_ALLOWED => {
            match method.as_str() {
                "GET" => {
                    // customize the response for GET requests
                    HttpResponse::MethodNotAllowed().json(json!({
                        "error": "method_not_allowed",
                        "message": "Sorry, GET requests are not allowed for this resource." // wtf john
                    }))
                },
                _ => {
                    HttpResponse::MethodNotAllowed().json(json!({
                        "error": "method_not_allowed",
                        "message": error_message
                    }))
                },
            }
        },
        StatusCode::REQUEST_TIMEOUT => {
            HttpResponse::RequestTimeout().json(json!({
                "error": "request_timeout",
                "message": error_message
            }))   
        },
        StatusCode::CONFLICT => {
            HttpResponse::Conflict().json(json!({
                "error": "conflict",
                "message": error_message
            }))
        },
        StatusCode::GONE => {
            HttpResponse::Gone().json(json!({
                "error": "gone",
                "message": error_message
            }))
        },
        StatusCode::LENGTH_REQUIRED => {
            HttpResponse::LengthRequired().json(json!({
                "error": "length_required",
                "message": error_message
            }))
        },
        StatusCode::PRECONDITION_FAILED => {
            HttpResponse::PreconditionFailed().json(json!({
                "error": "precondition_failed",
                "message": error_message
            }))
        },
        StatusCode::PAYLOAD_TOO_LARGE => {
            HttpResponse::PayloadTooLarge().json(json!({
                "error": "payload_too_large",
                "message": error_message
            }))
        },
        StatusCode::URI_TOO_LONG => {
            HttpResponse::UriTooLong().json(json!({
                "error": "uri_too_long",
                "message": error_message
            }))
        },
        StatusCode::UNSUPPORTED_MEDIA_TYPE => {
            HttpResponse::UnsupportedMediaType().json(json!({
                "error": "unsupported_media_type",
                "message": error_message
            }))
        },
        StatusCode::EXPECTATION_FAILED => HttpResponse::ExpectationFailed().body(error_message),
        _ => HttpResponse::InternalServerError().body("Internal Server Error"),
    };
    Ok(response)
}
