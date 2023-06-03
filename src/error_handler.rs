use actix_web::{
    error, 
    HttpResponse, 
    dev::ServiceRequest,
    http::StatusCode};

pub async fn not_found() -> HttpResponse { // error handler for unregistered requests
    HttpResponse::NotFound().body("Sorry, the requested resource could not be found.")
}

#[allow(unused_variables)]
#[allow(dead_code)]
pub fn handle_error(req: ServiceRequest, error: error::Error) -> actix_web::Result<HttpResponse> {
    let error_message = format!("{}", error);
    let response = match error.as_response_error().error_response().status() {
        StatusCode::BAD_REQUEST => HttpResponse::BadRequest().body(error_message),
        StatusCode::UNAUTHORIZED => HttpResponse::Unauthorized().body(error_message),
        StatusCode::FORBIDDEN => HttpResponse::Forbidden().body(error_message),
        StatusCode::NOT_FOUND => HttpResponse::NotFound().body(error_message),
        StatusCode::METHOD_NOT_ALLOWED => HttpResponse::MethodNotAllowed().body(error_message),
        StatusCode::REQUEST_TIMEOUT => HttpResponse::RequestTimeout().body(error_message),
        StatusCode::CONFLICT => HttpResponse::Conflict().body(error_message),
        StatusCode::GONE => HttpResponse::Gone().body(error_message),
        StatusCode::LENGTH_REQUIRED => HttpResponse::LengthRequired().body(error_message),
        StatusCode::PRECONDITION_FAILED => HttpResponse::PreconditionFailed().body(error_message),
        StatusCode::PAYLOAD_TOO_LARGE => HttpResponse::PayloadTooLarge().body(error_message),
        StatusCode::URI_TOO_LONG => HttpResponse::UriTooLong().body(error_message),
        StatusCode::UNSUPPORTED_MEDIA_TYPE => HttpResponse::UnsupportedMediaType().body(error_message),
        StatusCode::EXPECTATION_FAILED => HttpResponse::ExpectationFailed().body(error_message),
        _ => HttpResponse::InternalServerError().body("Internal Server Error"),
    };
    Ok(response)
}
