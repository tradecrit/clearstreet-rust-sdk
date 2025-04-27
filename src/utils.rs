use reqwest::{RequestBuilder, Response};
use crate::error::{Error, ErrorType};

#[tracing::instrument(skip(built_request), name = "exponential_backoff_request")]
pub async fn request(built_request: RequestBuilder) -> Result<Response, Error> {
    let mut delay_millis = 50;
    let max_retries = 5;

    for attempt in 1..=max_retries {
        let request_clone = built_request
            .try_clone()
            .ok_or_else(|| Error::new(ErrorType::InternalError, "Failed to clone request".into()))?;

        match request_clone.send().await {
            Ok(response) => return Ok(response),
            Err(error) => {
                tracing::warn!(%attempt, ?error, "Request failed, retrying after {}ms", delay_millis);

                if attempt == max_retries {
                    return Err(Error::new(ErrorType::TimeoutError, error.to_string()));
                }

                tokio::time::sleep(std::time::Duration::from_millis(delay_millis)).await;
                delay_millis = delay_millis.saturating_mul(2).min(500); // Cap the backoff delay
            }
        }
    }

    Err(Error::new(ErrorType::TimeoutError, "Exhausted retries".into()))
}

#[tracing::instrument(level="debug", skip(response), name="parse_response")]
pub async fn parse_response<T: serde::de::DeserializeOwned>(response: Response) -> Result<T, Error> {
    let text = response.text().await.map_err(|e| {
        tracing::error!("Error parsing response to text: {}", e);
        Error::new(ErrorType::ParseError, e.to_string())
    })?;

    tracing::debug!("Response: {}", text);

    match serde_json::from_str::<T>(&text) {
        Ok(parsed) => Ok(parsed),
        Err(e) => Err(e.into())
    }
}