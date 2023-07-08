use tokio::net::{TcpListener};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    // 绑定监听地址
    let addr = "127.0.0.1:6379";
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("服务地址: {:?}",addr);

    loop { // 针对多个监听连接循环
        let (mut socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            let mut buffer = [0u8; 8192];
            // let mut buffer = vec![0; 1024];
            loop {// 针对socket消息的切片循环
                match socket.read(&mut buffer[..]).await {
                    Ok(0) => {
                        println!("客户端断开连接");
                        return;
                    }
                    Ok(n) => {
                        println!("客户端消息: {}", String::from_utf8_lossy(&buffer[..n]));
                        // 原样发送回客户端
                        if let Err(e) = socket.write_all(&buffer[..n]).await {
                            eprintln!("Failed to send response: {}", e);
                            
                        }

                        // 如果需要可以主动关闭客户端连接
                        // socket.shutdown().await.unwrap();
                        // return;
                    }
                    Err(e) => {
                        eprintln!("Failed to read from socket: {}", e);
                        return;
                    }
                }
            }
        });
    }
}