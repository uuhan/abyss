use crate::AppResult;
use prometheus::{Encoder, Registry, TextEncoder};
use std::net::SocketAddr;
use tide::{http::mime, Request, Response};

/// Initializes the metrics context, and starts an HTTP server
/// to serve metrics.
pub async fn init_prometheus(addr: SocketAddr, registry: Registry) -> AppResult<()> {
    let mut app = tide::with_state(registry);

    app.at("/metrics").get(|req: Request<Registry>| async move {
        let registry = req.state();
        let metric_families = registry.gather();
        let mut buffer = vec![];
        let encoder = TextEncoder::new();
        encoder.encode(&metric_families, &mut buffer).unwrap();

        let response = Response::builder(200)
            .body(buffer)
            .header("Content-Type", encoder.format_type())
            .content_type(mime::HTML)
            .build();

        Ok(response)
    });

    app.listen(addr).await?;
    Ok(())
}
