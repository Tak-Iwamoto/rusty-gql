use futures_util::{pin_mut, AsyncRead, AsyncReadExt};
use serde::{Deserialize, Serialize};

use crate::variables::Variables;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    #[serde(default)]
    pub query: String,
    #[serde(default)]
    pub operation_name: Option<String>,

    #[serde(default)]
    pub variables: Variables,
}

pub async fn receive_http_request(
    body: impl AsyncRead + Send,
) -> Result<Request, HttpRequestError> {
    receive_json_body(body).await
}

pub async fn receive_json_body(body: impl AsyncRead) -> Result<Request, HttpRequestError> {
    let mut data = Vec::new();
    pin_mut!(body);

    body.read_to_end(&mut data)
        .await
        .map_err(HttpRequestError::Io)?;
    Ok(serde_json::from_slice::<Request>(&data)
        .map_err(|err| HttpRequestError::InvalidRequest(Box::new(err)))?)
}

#[derive(Debug)]
pub enum HttpRequestError {
    Io(std::io::Error),
    InvalidRequest(Box<dyn std::error::Error + Send + Sync>),
}
