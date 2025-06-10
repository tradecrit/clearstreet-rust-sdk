use reqwest::{Response};
use crate::error::{Error, ErrorType};

fn parse<T: serde::de::DeserializeOwned>(text: String) -> Result<T, Error> {
    tracing::debug!("Response: {:?}", text);

    match serde_json::from_str::<T>(&text) {
        Ok(parsed) => Ok(parsed),
        Err(e) => {
            tracing::error!("Error parsing response: {}", e);
            Err(e.into())
        }
    }
}

pub async fn parse_response<T: serde::de::DeserializeOwned>(response: Response) -> Result<T, Error> {
    let text = response.text().await.map_err(|e| {
        tracing::error!("Error parsing response to text: {}", e);
        Error::new(ErrorType::ParseError, &e.to_string())
    })?;

    parse(text)
}

#[cfg(feature = "sync")]
pub fn parse_response_blocking<T: serde::de::DeserializeOwned>(response: reqwest::blocking::Response) -> Result<T, Error> {
    let text = response.text().map_err(|e| {
        tracing::error!("Error parsing response to text: {}", e);
        Error::new(ErrorType::ParseError, &e.to_string())
    })?;

    parse(text)
}