use dioxus::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/send#:receiver")]
    Send { receiver: String },
    #[route("/receive")]
    Receive {},
}

const MAIN_CSS: Asset = asset!("/assets/main.css");
const BOOTSTRAP_CSS: Asset = asset!("/assets/bootstrap.min.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: BOOTSTRAP_CSS }
        Router::<Route> {}
    }
}

#[derive(Clone, Deserialize, Serialize)]
struct SendRequest {
    receiver: String,
    content: String,
}

async fn send_message(event: FormEvent) {
    event.prevent_default();

    let extract_value = |key| -> Option<String> {
        match event.get_first(key) {
            Some(FormValue::Text(text)) => Some(text),
            _ => None,
        }
    };

    let data = (|| {
        Some(SendRequest {
            receiver: extract_value("receiver")?,
            content: extract_value("text")?,
        })
    })();

    let client = reqwest::Client::new();
    client
        .post("http://localhost:9090/hi")
        .json(&data)
        .send()
        .await
        .unwrap();
}

/// Home page
#[component]
fn Send(receiver: String) -> Element {
    rsx! {
        div { class: "container",
            div { class: "d-flex vh-100 justify-content-center align-items-center",
                form { class: "col-10", onsubmit: send_message,
                    h2 { "Send a message to {receiver}!" },
                    input { type: "hidden", name: "receiver", value: "{receiver}" },
                    textarea { name: "text", class: "form-control mb-2" },
                    button { type: "submit", class: "btn btn-primary w-100", "Send!" }
                },
            }
        }
    }
}

/// Blog page
#[component]
pub fn Receive() -> Element {
    rsx! {
        div {
            "Dummy page"
        }
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link {
                to: Route::Send { receiver: "kek".to_string() },
                "Send"
            }
            Link {
                to: Route::Receive {},
                "Receive"
            }
        }

        Outlet::<Route> {}
    }
}
