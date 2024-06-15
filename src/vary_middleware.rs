use std::sync::Arc;

use axum::{extract::Request, middleware::Next, response::Response};
use axum_core::response::IntoResponse;
use futures::future::join_all;
use http::{
    header::{HeaderValue, VARY},
    Extensions,
};
use tokio::sync::oneshot::{self, Receiver, Sender};

use crate::{
    headers::{HX_REQUEST_STR, HX_TARGET_STR, HX_TRIGGER_NAME_STR, HX_TRIGGER_STR},
    HxError,
};

const MIDDLEWARE_DOUBLE_USE: &str =
    "Configuration error: `axum_httpx::vary_middleware` is used twice";

pub trait Notifier {
    fn sender(&mut self) -> Option<Sender<()>>;

    fn notify(&mut self) {
        if let Some(sender) = self.sender().take() {
            sender.send(()).ok();
        }
    }

    fn insert(extensions: &mut Extensions) -> Receiver<()>;
}

macro_rules! define_notifiers {
    ($($name:ident),*) => {
        $(
            #[derive(Clone)]
            pub(crate) struct $name(Option<Arc<Sender<()>>>);

            impl Notifier for $name {
                fn sender(&mut self) -> Option<Sender<()>> {
                    self.0.take().and_then(Arc::into_inner)
                }

                fn insert(extensions: &mut Extensions) -> Receiver<()> {
                    let (tx, rx) = oneshot::channel();
                    if extensions.insert(Self(Some(Arc::new(tx)))).is_some() {
                        panic!("{}", MIDDLEWARE_DOUBLE_USE);
                    }
                    rx
                }
            }
        )*
    }
}

define_notifiers!(
    HxRequestExtracted,
    HxTargetExtracted,
    HxTriggerExtracted,
    HxTriggerNameExtracted
);

pub async fn vary_middleware(mut request: Request, next: Next) -> Response {
    let exts = request.extensions_mut();
    let rx_header = [
        (HxRequestExtracted::insert(exts), HX_REQUEST_STR),
        (HxTargetExtracted::insert(exts), HX_TARGET_STR),
        (HxTriggerExtracted::insert(exts), HX_TRIGGER_STR),
        (HxTriggerNameExtracted::insert(exts), HX_TRIGGER_NAME_STR),
    ];

    let mut response = next.run(request).await;

    let used_headers: Vec<_> = join_all(
        rx_header
            .into_iter()
            .map(|(rx, header)| async move { rx.await.ok().map(|_| header) }),
    )
    .await
    .into_iter()
    .flatten()
    .collect();

    if used_headers.is_empty() {
        return response;
    }

    let value = match HeaderValue::from_str(&used_headers.join(", ")) {
        Ok(x) => x,
        Err(e) => return HxError::from(e).into_response(),
    };

    if let Err(e) = response.headers_mut().try_append(VARY, value) {
        return HxError::from(e).into_response();
    }

    response
}

#[cfg(test)]
mod tests {
    use axum::{routing::get, Router};

    use super::*;
    use crate::{HxRequest, HxTarget, HxTrigger, HxTriggerName};

    fn vary_headers(resp: &axum_test::TestResponse) -> Vec<HeaderValue> {
        resp.iter_headers_by_name("vary").cloned().collect()
    }

    fn server() -> axum_test::TestServer {
        let app = Router::new()
            .route("/no-extractors", get(|| async { () }))
            .route("/hx-request", get(|_: HxRequest| async { () }))
            .route("/hx-target", get(|_: HxTarget| async { () }))
            .route("/hx-trigger", get(|_: HxTrigger| async { () }))
            .route("/hx-trigger-name", get(|_: HxTriggerName| async { () }))
            .route(
                "/repeated-extractor",
                get(|_: HxRequest, _: HxRequest| async { () }),
            )
            .route(
                "/multiple-extractors",
                get(|_: HxRequest, _: HxTarget, _: HxTrigger, _: HxTriggerName| async { () }),
            )
            .layer(axum::middleware::from_fn(vary_middleware));
        axum_test::TestServer::new(app).unwrap()
    }

    #[tokio::test]
    async fn no_extractors() {
        assert!(vary_headers(&server().get("/no-extractors").await).is_empty());
    }

    #[tokio::test]
    async fn single_hx_request() {
        assert_eq!(
            vary_headers(&server().get("/hx-request").await),
            ["hx-request"]
        );
    }

    #[tokio::test]
    async fn single_hx_target() {
        assert_eq!(
            vary_headers(&server().get("/hx-target").await),
            ["hx-target"]
        );
    }

    #[tokio::test]
    async fn single_hx_trigger() {
        assert_eq!(
            vary_headers(&server().get("/hx-trigger").await),
            ["hx-trigger"]
        );
    }

    #[tokio::test]
    async fn single_hx_trigger_name() {
        assert_eq!(
            vary_headers(&server().get("/hx-trigger-name").await),
            ["hx-trigger-name"]
        );
    }

    #[tokio::test]
    async fn repeated_extractor() {
        assert_eq!(
            vary_headers(&server().get("/repeated-extractor").await),
            ["hx-request"]
        );
    }

    // Extractors can be used multiple times e.g. in middlewares
    #[tokio::test]
    async fn multiple_extractors() {
        assert_eq!(
            vary_headers(&server().get("/multiple-extractors").await),
            ["hx-request, hx-target, hx-trigger, hx-trigger-name"],
        );
    }
}
