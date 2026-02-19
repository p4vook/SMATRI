mod util;

use axum::{
    routing,
    Router,
    Json
};

use http::StatusCode;
use serde::{
    Serialize,
    Deserialize
};

async fn handler() -> &'static str {
    "Hello, world!\n"
}

type RecepientID = String;

#[derive(Clone, Serialize, Deserialize)]
struct Message {
    recepient: RecepientID,
    text: String
}

#[derive(Clone, Serialize, Deserialize)]
struct ReceiveRequest {
    recepient: RecepientID
}

async fn send_message(message: Json<Message>) -> StatusCode {
    println!("Received send_message (recepient={:#?}, text={:#?})", message.recepient, message.text);

    StatusCode::OK
}

async fn receive_messages(receive_request: Json<ReceiveRequest>) -> Json<Vec<String>> {
    Json(Vec::new())
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", routing::get(handler))
        .route("/send", routing::post(send_message))
        .route("/receive", routing::post(receive_messages));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}
