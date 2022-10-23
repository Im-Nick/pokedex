use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use poke_api::errors::{Error, ErrorStatus};

#[derive(Debug, Display)]
pub enum MyError {
    #[display(fmt = "Bad Request: {}", _0)]
    BadRequest(String),
    #[display(fmt = "Internal server error")]
    InternalServerError,
    #[display(fmt = "Failed to decode body")]
    DecodingError,
    #[display(fmt = "Too many requests!")]
    TooManyRequests,
}

impl ResponseError for MyError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        match self {
            MyError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            MyError::InternalServerError => HttpResponse::InternalServerError().finish(),
            MyError::DecodingError => {
                log::error!("Failed to decode body");
                HttpResponse::InternalServerError().finish()
            }
            MyError::TooManyRequests => HttpResponse::TooManyRequests().finish(),
        }
    }
}

impl From<Error> for MyError {
    fn from(error: Error) -> Self {
        match error.status {
            ErrorStatus::BadRequest => Self::BadRequest("Pokemon not found!".to_string()),
            ErrorStatus::DecodingError => Self::DecodingError,
            ErrorStatus::RequestTimeout => Self::InternalServerError,
            ErrorStatus::TooManyRequests => Self::TooManyRequests,
            ErrorStatus::InternalServerError => Self::InternalServerError,
        }
    }
}
