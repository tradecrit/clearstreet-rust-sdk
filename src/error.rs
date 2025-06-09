use std::fmt::Display;
use reqwest::header::InvalidHeaderValue;
use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite;
use crate::orders::OrderState;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[allow(clippy::enum_variant_names)]
pub enum ErrorType {
    AuthenticationError,
    ParseError,
    ThirdPartyError,
    InternalError,
    TimeoutError,
    IoError,
    ArithmeticError,
    HttpError,
    SerializationError,
    NotFound,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub error_type: ErrorType,
    pub message: String,
}

impl Error {
    pub(crate) fn internal(p0: &str) -> Result<OrderState, Error> {
        Err(Error::new(ErrorType::InternalError, p0))
    }
}

impl Error {
    pub fn new(error_type: ErrorType, message: &str) -> Self {
        Error {
            error_type,
            message: message.to_string(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.error_type, self.message)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }
}

impl From<InvalidHeaderValue> for Error {
    fn from(err: InvalidHeaderValue) -> Self {
        Error::new(ErrorType::SerializationError, &err.to_string())
    }
}


impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::new(ErrorType::ParseError, &err.to_string())
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::new(ErrorType::IoError, &err.to_string())
    }
}

impl From<tungstenite::error::Error> for Error {
    fn from(err: tungstenite::error::Error) -> Self {
        Error::new(ErrorType::IoError, &err.to_string())
    }
}
