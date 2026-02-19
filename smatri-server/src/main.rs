mod util;

use axum::{
    routing,
    Router
};

async fn handler() -> &'static str {
    "Hello, world!\n"
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", routing::get(handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}
