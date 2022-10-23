#[derive(Debug)]
pub struct Error {
    pub status: ErrorStatus,
    pub message: String,
}

#[derive(Debug, Clone)]
pub enum ErrorStatus {
    BadRequest,
    DecodingError,
    RequestTimeout,
    TooManyRequests,
    InternalServerError,
}

impl Error {
    pub fn new(status: ErrorStatus, message: String) -> Self {
        Self {
            status: status,
            message: message,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Error {
        let error_message = error.to_string();
        if error.is_decode() {
            Error::new(ErrorStatus::DecodingError, error_message)
        } else if error.is_request() {
            Error::new(ErrorStatus::BadRequest, error_message)
        } else if error.is_timeout() {
            Error::new(ErrorStatus::BadRequest, error_message)
        } else if error.is_status() {
            match error.status() {
                Some(status) => match status.as_u16() {
                    429 => Error::new(ErrorStatus::TooManyRequests, error_message),
                    _ => Error::new(ErrorStatus::BadRequest, error_message),
                },
                None => Error::new(ErrorStatus::InternalServerError, error_message),
            }
        } else {
            Error::new(ErrorStatus::InternalServerError, error_message)
        }
    }
}

impl From<reqwest_middleware::Error> for Error {
    fn from(error: reqwest_middleware::Error) -> Self {
        match error {
            reqwest_middleware::Error::Middleware(e) => {
                Error::new(ErrorStatus::InternalServerError, e.to_string())
            }
            reqwest_middleware::Error::Reqwest(e) => Error::from(e),
        }
    }
}
