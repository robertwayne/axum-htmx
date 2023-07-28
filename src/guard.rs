//! Request guard for protecting a router against non-htmx requests.

use std::{
    fmt,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use axum::{
    http::{Request, StatusCode},
    response::Response,
};
use futures_core::ready;
use pin_project_lite::pin_project;
use tower::{Layer, Service};

use crate::HX_REQUEST;

/// Checks if the request contains the `HX-Request` header, returning a `403:
/// Forbidden` response if the header is not present.
///
/// This can be used to protect routes that should only be accessed via htmx
/// requests.
#[derive(Default, Debug, Clone)]
pub struct HxRequestGuardLayer;

impl HxRequestGuardLayer {
    #[allow(clippy::default_constructed_unit_structs)]
    pub fn new() -> Self {
        Self::default()
    }
}

impl<S> Layer<S> for HxRequestGuardLayer {
    type Service = HxRequestGuard<S>;

    fn layer(&self, inner: S) -> Self::Service {
        HxRequestGuard {
            inner,
            hx_request: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HxRequestGuard<S> {
    inner: S,
    hx_request: bool,
}

impl<S, T, U> Service<Request<T>> for HxRequestGuard<S>
where
    S: Service<Request<T>, Response = Response<U>>,
    U: Default,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = ResponseFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<T>) -> Self::Future {
        // This will always contain a "true" value.
        if req.headers().contains_key(HX_REQUEST) {
            self.hx_request = true;
        }

        let response_future = self.inner.call(req);

        ResponseFuture {
            response_future,
            hx_request: self.hx_request,
        }
    }
}

pin_project! {
    pub struct ResponseFuture<F> {
        #[pin]
        response_future: F,
        hx_request: bool,
    }
}

impl<F, B, E> Future for ResponseFuture<F>
where
    F: Future<Output = Result<Response<B>, E>>,
    B: Default,
{
    type Output = Result<Response<B>, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let response: Response<B> = ready!(this.response_future.poll(cx))?;

        match *this.hx_request {
            true => Poll::Ready(Ok(response)),
            false => {
                let mut res = Response::new(B::default());
                *res.status_mut() = StatusCode::FORBIDDEN;

                Poll::Ready(Ok(res))
            }
        }
    }
}

#[derive(Debug, Default)]
struct HxRequestGuardError;

impl fmt::Display for HxRequestGuardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("HxRequestGuardError")
    }
}

impl std::error::Error for HxRequestGuardError {}
