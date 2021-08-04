use super::{FromRequest, RequestParts};
use async_trait::async_trait;
use std::convert::Infallible;

/// Extractor that extracts the raw query string, without parsing it.
///
/// # Example
///
/// ```rust,no_run
/// use axum::prelude::*;
/// use futures::StreamExt;
///
/// async fn handler(extract::RawQuery(query): extract::RawQuery) {
///     // ...
/// }
///
/// let app = route("/users", get(handler));
/// # async {
/// # axum::Server::bind(&"".parse().unwrap()).serve(app.into_make_service()).await.unwrap();
/// # };
/// ```
#[derive(Debug)]
pub struct RawQuery(pub Option<String>);

#[async_trait]
impl<B> FromRequest<B> for RawQuery
where
    B: Send,
{
    type Rejection = Infallible;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let query = req
            .uri()
            .and_then(|uri| uri.query())
            .map(|query| query.to_string());
        Ok(Self(query))
    }
}
