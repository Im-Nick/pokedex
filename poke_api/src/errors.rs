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
    fn from(e: reqwest::Error) -> Error {
        if e.is_decode() {
            Error::new(ErrorStatus::DecodingError, e.to_string())
        } else if e.is_request() {
            Error::new(ErrorStatus::BadRequest, e.to_string())
        } else if e.is_timeout() {
            Error::new(ErrorStatus::BadRequest, e.to_string())
        } else if e.is_status() {
            match e.status() {
                Some(status) => match status.as_u16() {
                    429 => Error::new(ErrorStatus::TooManyRequests, e.to_string()),
                    _ => Error::new(ErrorStatus::BadRequest, e.to_string()),
                },
                None => Error::new(ErrorStatus::InternalServerError, e.to_string()),
            }
        } else {
            Error::new(ErrorStatus::InternalServerError, e.to_string())
        }
    }
}

impl From<reqwest_middleware::Error> for Error {
    fn from(e: reqwest_middleware::Error) -> Self {
        match e {
            reqwest_middleware::Error::Middleware(e) => {
                Error::new(ErrorStatus::InternalServerError, e.to_string())
            }
            reqwest_middleware::Error::Reqwest(e) => Error::from(e),
        }
    }
}
