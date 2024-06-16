//! Using `auto-vary` middleware
//!
//! Don't forget about the feature while running it:
//! `cargo run --features auto-vary --example auto-vary`
use std::time::Duration;

use axum::{response::Html, routing::get, serve, Router};
use axum_htmx::{AutoVaryLayer, HxRequest};
use tokio::{net::TcpListener, time::sleep};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        // Add the middleware
        .layer(AutoVaryLayer);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    serve(listener, app).await.unwrap();
}

// Our handler differentiates full-page GET requests from HTMx-based ones by looking at the `hx-request`
// requestheader.
//
// The middleware sees the usage of the `HxRequest` extractor and automatically adds the
// `Vary: hx-request` response header.
async fn handler(HxRequest(hx_request): HxRequest) -> Html<&'static str> {
    if hx_request {
        // For HTMx-based GET request, it returns a partial page update
        sleep(Duration::from_secs(3)).await;
        return Html("HTMx response");
    }
    // While for a normal GET request, it returns the whole page
    Html(
        r#"
        <script src="https://unpkg.com/htmx.org@1"></script>
        <p hx-get="/" hx-trigger="load">Loading ...</p>
        "#,
    )
}
