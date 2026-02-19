mod util;

use axum::{
    routing,
    Router,
    Json,
    extract::{State}
};
use http::StatusCode;
use serde::{
    Serialize,
    Deserialize
};
use rusqlite;
use std::sync::{Arc, Mutex};

type RecepientID = String;

#[derive(Clone, Serialize, Deserialize)]
struct Message {
    recipient: RecepientID,
    text: String
}

#[derive(Clone, Serialize, Deserialize)]
struct ReceiveRequest {
    recipient: RecepientID
}

#[derive(Clone)]
struct AppState {
    connection: Arc<Mutex<rusqlite::Connection>>
}

impl AppState {
    fn new() -> Self {
        Self {
            connection: Arc::new(Mutex::new(
                rusqlite::Connection::open("testing.sqlite").unwrap()))
        }
    }

    fn init(self: &mut AppState) -> Result<(), rusqlite::Error> {
        self.connection.lock().unwrap().execute(
            "CREATE TABLE IF NOT EXISTS message (
                id          INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                recipient   BLOB NOT NULL,
                content     BLOB NOT NULL
            )",
            (),
        ).map(|_| ())
    }
}

async fn send_message(State(state): State<AppState>, message: Json<Message>) -> (StatusCode, String) {
    match state.connection.lock().unwrap().execute(
        "INSERT INTO message (recipient, content) VALUES (?1, ?2)",
        (&message.recipient, &message.text),
    ) {
        Ok(_) => (StatusCode::OK, String::new()),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("error while inserting: {}", e))
    }
}

async fn receive_messages(State(state): State<AppState>, receive_request: Json<ReceiveRequest>) -> Result<Json<Vec<String>>, (StatusCode, String)> {
    (|| {
        let conn = state.connection.lock().unwrap();

        let mut stmt = conn.prepare(
            "SELECT content FROM message WHERE recipient = ?1",
        )?;
        
        let rows = stmt.query_map(
            [receive_request.recipient.as_str()],
            |row| -> Result<String, _> { row.get(0) }
        )?;

        let mut contents = Vec::new();
        for content in rows {
            contents.push(content?);
        }
        
        Ok(Json(contents))
    })().map_err(|e: rusqlite::Error| (StatusCode::INTERNAL_SERVER_ERROR, format!("error while querying: {}", e)))
}

#[tokio::main]
async fn main() {
    let mut state = AppState::new();
    state.init().unwrap();

    let app = Router::new()
        .route("/send", routing::post(send_message))
        .route("/receive", routing::post(receive_messages))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}
