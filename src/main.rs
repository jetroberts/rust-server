mod server;
use std::net::SocketAddr;
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/ws", get(server::ws_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
