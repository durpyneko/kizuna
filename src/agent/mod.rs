mod routes;

use axum::{
    Router,
    extract::Request,
    middleware::{self, Next},
    response::Response,
    routing::{get, post},
};
use tokio::net::TcpListener;

const START_PORT: u16 = 4989;

pub async fn start() -> std::io::Result<()> {
    let app = Router::new()
        .route("/", get(routes::root))
        .route("/status", get(routes::status))
        .route("/self-update", post(routes::self_update))
        .layer(middleware::from_fn(log_requests));

    let listener = bind_available_port(START_PORT).await?;

    log::info!("Agent listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await
}

async fn bind_available_port(start: u16) -> std::io::Result<TcpListener> {
    let mut port = start;

    loop {
        match TcpListener::bind(("0.0.0.0", port)).await {
            Ok(listener) => return Ok(listener),

            Err(e) if e.kind() == std::io::ErrorKind::AddrInUse => {
                port += 1;
            }

            Err(e) => return Err(e),
        }
    }
}

async fn log_requests(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let path = req.uri().path().to_owned();

    let response = next.run(req).await;

    log::info!("{} {} -> {}", method, path, response.status());

    response
}
