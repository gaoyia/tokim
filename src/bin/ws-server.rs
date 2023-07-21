use futures::{SinkExt, StreamExt};
use tokio::net::{TcpListener};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;
#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Listening on: ws://{}", addr);

    loop {
        let (stream, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let mut websocket = accept_async(stream).await.unwrap();

            println!("WebSocket connection established");

            while let Some(message) = websocket.next().await {
                match message {
                    Ok(Message::Text(text)) => {
                        println!("Received message: {}", text);

                        websocket.send(Message::text("Hello from server!")).await.unwrap();
                    }
                    Ok(Message::Close(_)) => {
                        println!("WebSocket connection closed:");
                        break; // 当收到关闭连接的消息时，跳出循环
                    }
                    Ok(Message::Binary(bin)) => {
                        println!("bin:{:?}",bin);
                    }
                    Ok(Message::Ping(bin)) => {
                        println!("bin:{:?}",bin);
                    }
                    Ok(Message::Pong(bin)) => {
                        println!("bin:{:?}",bin);
                    }
                    Err(e) => {
                        eprintln!("WebSocket error: {}", e);
                        break;
                    }
                }
            }
        });
    }
}