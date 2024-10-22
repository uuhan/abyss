use crate::endpoints::http::*;
use std::time::Instant;
use tide::{Middleware, Next, Request};

/// Log all incoming requests and responses with tracing spans.
#[derive(Debug, Default, Clone)]
pub struct TraceMiddleware;

impl TraceMiddleware {
    /// Create a new instance of `TraceMiddleware`.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Log a request and a response.
    async fn log<'a, State: Clone + Send + Sync + 'static>(
        &'a self,
        ctx: Request<State>,
        next: Next<'a, State>,
    ) -> tide::Result {
        let path = ctx.url().path().to_owned();
        let method = ctx.method();

        // Ok(async {
        //     tracing::info!("received");
        //     let start = Instant::now();
        //     let response = next.run(ctx).await;
        //     let duration = start.elapsed();
        //     let status = response.status();
        //
        //     info_span!("Response", http.status_code = status as u16, http.duration = ?duration)
        //         .in_scope(|| {
        //             if status.is_server_error() {
        //                 let span = error_span!("Internal error", error = field::Empty);
        //                 if let Some(error) = response.error() {
        //                     span.record("error", &field::display(error));
        //                 }
        //                 span.in_scope(|| tracing::error!("sent"));
        //             } else if status.is_client_error() {
        //                 warn_span!("Client error").in_scope(|| tracing::warn!("sent"));
        //             } else {
        //                 tracing::info!("sent")
        //             }
        //         });
        //     response
        // }
        // .instrument(info_span!("Request", http.method = %method, http.target = %path))
        // .await)

        let start = Instant::now();
        let response = next.run(ctx).await;
        let duration = start.elapsed();
        let status = response.status();

        if status.is_server_error() {
            tracing::error!(
                reason = "server error.",
                error = ?response.error(),
                http.method = %method,
                http.target = %path,
                http.status_code = status as u16,
                http.duration = ?duration,
                "[http/server-error]"
            );
        } else if status.is_client_error() {
            tracing::error!(
                reason = "client error.",
                error = ?response.error(),
                http.method = %method,
                http.target = %path,
                http.status_code = status as u16,
                http.duration = ?duration,
                "[http/client-error]"
            );
        }

        tracing::info!(
            "Request{{http.method={} http.target={}}}:Response{{http.status={} http.duration={:?}}}",
            method,
            path,
            status as u16,
            duration,
        );

        Ok(response)
    }
}

#[async_trait::async_trait]
impl<State: HttpStateT> Middleware<State> for TraceMiddleware {
    async fn handle(&self, req: Request<State>, next: Next<'_, State>) -> tide::Result {
        self.log(req, next).await
    }
}
