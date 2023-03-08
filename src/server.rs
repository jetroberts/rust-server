use std::{net::SocketAddr};
use axum::{extract::{ws::{WebSocket, WebSocketUpgrade, Message, self}}, response::{IntoResponse, Html}, Router, routing::get, http::Response};
use tokio::sync::{mpsc::{self, Receiver}};

#[derive(Clone, Debug)]
struct State {
    x: Vec<f32>,
    y: Vec<f32>
}

pub async fn run_server() {

    let app = Router::new()
        .route("/", get(root))
        .route("/index.css", get(styling))
        .route("/index.js", get(js))
        .route("/api/ws", get(ws_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr).serve(app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}

async fn root() -> impl IntoResponse {
    let main = tokio::fs::read("src/index.html").await.unwrap();
    Html(main)
}

async fn styling() -> impl IntoResponse {
    let style = tokio::fs::read_to_string("src/index.css").await.unwrap();

    let resp = Response::builder()
        .header("content-type", "text/css;charset=utf-8")
        .body(style);

    match resp { 
        Ok(style) => style,
        Err(err) => {
            println!("Got an error trying to get the style sheet {}", err);
            Response::default()
        },
    }
}

async fn js() -> impl IntoResponse {
    let js = tokio::fs::read_to_string("src/index.js").await.unwrap();

    let resp = Response::builder()
        .header("content-type", "application/javascript;charset=utf-8")
        .body(js);

    match resp { 
        Ok(js) => js,
        Err(err) => {
            println!("Got an error trying to get the js file {}", err);
            Response::default()
        },
    }
}

async fn ws_handler(
    ws: WebSocketUpgrade
) -> impl IntoResponse {
    let (tx, rx) = mpsc::channel(1);
    let mut s = State{x: vec![1.2], y: vec![12.2]};
    tokio::spawn(async move {
        loop {
            if tx.send(s.clone()).await.is_err() {
                println!("no receiver");
                return
            };
            s.x[0] += 1.0;

            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        };
    });

    ws.on_upgrade( move |socket| handle_socket(socket, rx))
}

async fn handle_socket(mut socket: WebSocket, mut rx: Receiver<State>) {
    tokio::spawn(async move {
        loop {
            let msg = rx.recv().await;
            match msg {
                Some(m) => {
                    if socket.send(Message::Text(format!("send {:?}", m.x))).await.is_err() {
                        println!("connection closed");
                        return
                    };
                },
                None => return,
            }
        }
    });
}
