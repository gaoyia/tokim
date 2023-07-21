use futures::{SinkExt, StreamExt};
use tokio::net::{TcpListener};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;
#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("监听地址: ws://{}", addr);

    loop {
        let (stream, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let mut websocket = accept_async(stream).await.unwrap();

            println!("WebSocket 连接成功");

            while let Some(message) = websocket.next().await {
                match message {
                    Ok(Message::Text(text)) => {
                        println!("文本消息: {}", text);
                        websocket.send(Message::text("Hello from server!")).await.unwrap();
                        // websocket.send(Message::Ping([0x1].to_vec())).await.unwrap(); // 发送ping消息
                    }
                    Ok(Message::Close(reason)) => {
                        let res = reason.unwrap();
                        println!("WebSocket连接关闭,关闭原因:{:?},关闭代码{:?}",res.reason,res.code);
                        break; // 当收到关闭连接的消息时，跳出循环
                    }
                    Ok(Message::Binary(bin)) => {
                        // 二进制数据
                        println!("Binary:{:?}",bin);
                    }
                    Ok(Message::Ping(ping)) => {
                        //返回pong
                        websocket.send(Message::Pong(ping)).await.unwrap();
                    }
                    Ok(Message::Pong(pong)) => {
                        // 收到pong响应当前连接正常
                        println!("Pong:{:?}",pong);
                    }
                    Err(e) => {
                        eprintln!("WebSocket 错误: {}", e);
                        break;
                    }
                }
            }
        });
    }
}