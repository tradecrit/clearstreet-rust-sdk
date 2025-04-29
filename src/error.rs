use std::fmt::Display;
use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite;

#[derive(Serialize, Deserialize, Debug)]
pub struct BrokerApiError {
    #[serde(rename = "type")]
    pub error_type: String,
    pub title: String,
    pub status: i64,
    pub detail: String,
    pub instance: String,
}

impl Display for BrokerApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error {}: {} - {}", self.status, self.title, self.detail)
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum ErrorType {
    ParseError,
    ThirdPartyError,
    InternalError,
    TimeoutError,
    IoError,
    ArithmeticError,
    HttpError
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub error_type: ErrorType,
    pub message: String,
}

impl Error {
    pub fn new(error_type: ErrorType, message: String) -> Self {
        Error {
            error_type,
            message,
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


impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::new(ErrorType::ParseError, err.to_string())
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::new(ErrorType::IoError, err.to_string())
    }
}

impl From<tungstenite::error::Error> for Error {
    fn from(err: tungstenite::error::Error) -> Self {
        Error::new(ErrorType::IoError, err.to_string())
    }
}
