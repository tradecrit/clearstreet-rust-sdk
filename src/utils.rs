use reqwest::{Response};
use crate::error::{Error, ErrorType};

pub async fn parse_response<T: serde::de::DeserializeOwned>(response: Response) -> Result<T, Error> {
    let text = response.text().await.map_err(|e| {
        tracing::error!("Error parsing response to text: {}", e);
        Error::new(ErrorType::ParseError, e.to_string())
    })?;
    
    match serde_json::from_str::<T>(&text) {
        Ok(parsed) => Ok(parsed),
        Err(e) => {
            tracing::error!("Error parsing response: {}", e);
            Err(e.into())
        }
    }
}
