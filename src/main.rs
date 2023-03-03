use std::net::SocketAddr;

use axum::{extract::{ConnectInfo, WebSocketUpgrade, ws::{WebSocket, Message}}, response::IntoResponse, routing::get, Router, TypedHeader, headers};

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/", get(root))
        .route("/ws", get(ws_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    user: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let cur_user  = match user {
        Some(user) => user.to_string(),
        None => "unknown".to_string(),
    };

    println!("{cur_user} at {addr} connected");
    ws.on_upgrade(move |socket| handle_socket(socket, addr)) 
}

async fn handle_socket(mut socket: WebSocket, who: SocketAddr) {
    if socket.send(Message::Ping(vec![2, 3, 4])).await.is_ok() {
        println!("Sent initial ping");
    } else {
        println!("Unable to ping client {who}");
        return
    }

    tokio::spawn(async move {
        let q_msg = 50;
        for i in 0..q_msg {
            if socket.send(Message::Text(format!("Sent message {i} from server"))).await.is_err() {
                return i;
            }

            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
        q_msg
    });
}
