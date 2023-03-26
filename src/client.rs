use async_std::io::prelude::BufReadExt;
use async_std::stream::StreamExt;

use bytes::Bytes;

use futures::TryStreamExt;

use serde::de::DeserializeOwned;

use crate::error::{Error, Result};

#[derive(Clone)]
pub struct LichessApi<HttpClient> {
    pub client: HttpClient,
    bearer_auth: Option<String>,
}

impl<HttpClient> LichessApi<HttpClient> {
    pub fn new(client: HttpClient, auth_token: Option<String>) -> Self {
        let bearer_auth = auth_token.map(|token| format!("Bearer {}", token));
        Self {
            client,
            bearer_auth,
        }
    }

    pub(crate) async fn expect_one_model<Model, G>(&self, stream: &mut G) -> Result<Model>
    where
        G: StreamExt<Item = Result<Model>> + std::marker::Unpin,
    {
        stream
            .next()
            .await
            .ok_or(Error::Response("empty response stream".to_string()))?
    }
}

impl LichessApi<reqwest::Client> {
    pub(crate) async fn send<Model: DeserializeOwned>(
        &self,
        mut http_request: http::Request<Bytes>,
    ) -> Result<impl StreamExt<Item = Result<Model>>> {
        if let Some(auth) = &self.bearer_auth {
            let auth_header = http::HeaderValue::from_str(&auth)
                .map_err(|e| Error::HttpRequestBuilder(http::Error::from(e)))?;
            http_request.headers_mut().insert(http::header::AUTHORIZATION, auth_header);
        };

        let convert_err = |e: reqwest::Error| Error::Request(e.to_string());
        let mut request = reqwest::Request::try_from(http_request).map_err(convert_err)?;
        *request.timeout_mut() = None;

        let stream = self
            .client
            .execute(request)
            .await
            .map_err(convert_err)?
            .bytes_stream()
            .map_err(|e| futures::io::Error::new(futures::io::ErrorKind::Other, e))
            .into_async_read()
            .lines()
            .filter(|l| {
                // To avoid trying to serialize blank keep alive lines.
                !l.as_ref().unwrap_or(&"".to_string()).is_empty()
            })
            .map(|l| -> Result<Model> {
                let line = l?;
                if line.starts_with("<!DOCTYPE html>") {
                    return Err(crate::error::Error::PageNotFound());
                }
                serde_json::from_str(&line).map_err(|e| crate::error::Error::Json(e))
            });

        Ok(stream)
    }
}
