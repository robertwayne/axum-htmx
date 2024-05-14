use std::sync::Arc;

use axum::{extract::Request, middleware::Next, response::Response};
use axum_core::response::IntoResponse;
use http::{
    header::{HeaderValue, VARY},
    Extensions,
};
use tokio::sync::oneshot::{self, Receiver, Sender};

use crate::{
    headers::{HX_REQUEST_STR, HX_TARGET_STR},
    HxError,
};

const MIDDLEWARE_DOUBLE_USE: &str =
    "Configuration error: `axum_httpx::vary_middleware` is used twice";

#[derive(Clone)]
pub(crate) struct HxRequestExtracted(Option<Arc<Sender<()>>>);

#[derive(Clone)]
pub(crate) struct HxTargetExtracted(Option<Arc<Sender<()>>>);

pub trait Notifier {
    fn sender(&mut self) -> Option<Sender<()>>;

    fn notify(&mut self) {
        if let Some(sender) = self.sender().take() {
            sender.send(()).ok();
        }
    }
}

impl Notifier for HxRequestExtracted {
    fn sender(&mut self) -> Option<Sender<()>> {
        self.0.take().and_then(Arc::into_inner)
    }
}

impl Notifier for HxTargetExtracted {
    fn sender(&mut self) -> Option<Sender<()>> {
        self.0.take().and_then(Arc::into_inner)
    }
}

impl HxRequestExtracted {
    fn insert_into_extensions(extensions: &mut Extensions) -> Receiver<()> {
        let (tx, rx) = oneshot::channel();
        if extensions.insert(Self(Some(Arc::new(tx)))).is_some() {
            panic!("{}", MIDDLEWARE_DOUBLE_USE);
        }
        rx
    }
}

impl HxTargetExtracted {
    fn insert_into_extensions(extensions: &mut Extensions) -> Receiver<()> {
        let (tx, rx) = oneshot::channel();
        if extensions.insert(Self(Some(Arc::new(tx)))).is_some() {
            panic!("{}", MIDDLEWARE_DOUBLE_USE);
        }
        rx
    }
}

pub async fn vary_middleware(mut request: Request, next: Next) -> Response {
    let hx_request_rx = HxRequestExtracted::insert_into_extensions(request.extensions_mut());
    let hx_target_rx = HxTargetExtracted::insert_into_extensions(request.extensions_mut());

    let mut response = next.run(request).await;

    let mut used = Vec::with_capacity(4);
    if hx_request_rx.await.is_ok() {
        used.push(HX_REQUEST_STR)
    }
    if hx_target_rx.await.is_ok() {
        used.push(HX_TARGET_STR)
    }

    if !used.is_empty() {
        let value = match HeaderValue::from_str(&used.join(", ")) {
            Ok(x) => x,
            Err(e) => return HxError::from(e).into_response(),
        };
        if let Err(e) = response.headers_mut().try_append(VARY, value) {
            return HxError::from(e).into_response();
        }
    }

    response
}

#[cfg(test)]
mod tests {
    use axum::{routing::get, Router};

    use super::*;
    use crate::{HxRequest, HxTarget};

    fn vary_headers(resp: &axum_test::TestResponse) -> Vec<HeaderValue> {
        resp.iter_headers_by_name("vary").cloned().collect()
    }

    #[tokio::test]
    async fn multiple_headers() {
        let app = Router::new()
            .route("/no-extractors", get(|| async { () }))
            .route("/single-extractor", get(|_: HxRequest| async { () }))
            // Extractors can be used multiple times e.g. in middlewares
            .route(
                "/repeated-extractor",
                get(|_: HxRequest, _: HxRequest| async { () }),
            )
            .route(
                "/multiple-extractors",
                get(|_: HxRequest, _: HxTarget| async { () }),
            )
            .layer(axum::middleware::from_fn(vary_middleware));
        let server = axum_test::TestServer::new(app).unwrap();

        assert!(vary_headers(&server.get("/no-extractors").await).is_empty());
        assert_eq!(
            vary_headers(&server.get("/single-extractor").await),
            [HX_REQUEST_STR]
        );
        assert_eq!(
            vary_headers(&server.get("/repeated-extractor").await),
            [HX_REQUEST_STR]
        );
        assert_eq!(
            vary_headers(&server.get("/multiple-extractors").await),
            [format!("{HX_REQUEST_STR}, {HX_TARGET_STR}")]
        );
    }
}
