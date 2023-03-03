use std::net::SocketAddr;

use axum::{extract::{WebSocketUpgrade, ConnectInfo, ws::{WebSocket, Message}}, TypedHeader, headers, response::IntoResponse};

pub async fn ws_handler(
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
        let mut index = 0;
        loop { 
            

            if socket.send(Message::Text(format!("Sent {index}"))).await.is_err() { 
                return
            }
            index += 1;

            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    });
}
