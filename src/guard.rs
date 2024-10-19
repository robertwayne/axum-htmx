//! Request guard for protecting a router against non-htmx requests.

use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use futures_core::ready;
use http::{header::LOCATION, response::Response, Request, StatusCode};
use pin_project_lite::pin_project;
use tower::{Layer, Service};

use crate::HX_REQUEST;

/// Checks if the request contains the `HX-Request` header, redirecting to the
/// given location if not.
///
/// This can be useful for preventing users from accidently ending up on a route
/// which would otherwise return only partial HTML data.
#[derive(Debug, Clone)]
pub struct HxRequestGuardLayer<'a> {
    redirect_to: &'a str,
}

impl<'a> HxRequestGuardLayer<'a> {
    pub fn new(redirect_to: &'a str) -> Self {
        Self { redirect_to }
    }
}

impl Default for HxRequestGuardLayer<'_> {
    fn default() -> Self {
        Self { redirect_to: "/" }
    }
}

impl<'a, S> Layer<S> for HxRequestGuardLayer<'a> {
    type Service = HxRequestGuard<'a, S>;

    fn layer(&self, inner: S) -> Self::Service {
        HxRequestGuard {
            inner,
            hx_request: false,
            layer: self.clone(),
        }
    }
}

/// Tower service that implements redirecting to non-partial routes.
#[derive(Debug, Clone)]
pub struct HxRequestGuard<'a, S> {
    inner: S,
    hx_request: bool,
    layer: HxRequestGuardLayer<'a>,
}

impl<'a, S, T, U> Service<Request<T>> for HxRequestGuard<'a, S>
where
    S: Service<Request<T>, Response = Response<U>>,
    U: Default,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = private::ResponseFuture<'a, S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<T>) -> Self::Future {
        // This will always contain a "true" value.
        if req.headers().contains_key(HX_REQUEST) {
            self.hx_request = true;
        }

        let response_future = self.inner.call(req);

        private::ResponseFuture {
            response_future,
            hx_request: self.hx_request,
            layer: self.layer.clone(),
        }
    }
}

mod private {
    use super::*;

    pin_project! {
        pub struct ResponseFuture<'a, F> {
            #[pin]
            pub(super) response_future: F,
            pub(super) hx_request: bool,
            pub(super) layer: HxRequestGuardLayer<'a>,
        }
    }

    impl<F, B, E> Future for ResponseFuture<'_, F>
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
                    let res = Response::builder()
                        .status(StatusCode::SEE_OTHER)
                        .header(LOCATION, this.layer.redirect_to)
                        .body(B::default())
                        .expect("failed to build response");

                    Poll::Ready(Ok(res))
                }
            }
        }
    }
}
