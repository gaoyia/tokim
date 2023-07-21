use tokio::time::{self, sleep};
use tokio_tungstenite::connect_async;
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use tokio_tungstenite::tungstenite::Message;

async fn create_wss_client(uri: String) -> Result<(), Box<dyn std::error::Error>> {
    let (ws_stream, _) = connect_async(uri).await?;
    let (mut write, mut read) = ws_stream.split();
    loop {
        write.send(Message::Text("Ping".to_string())).await?;
        sleep(time::Duration::from_secs(1)).await;
        while let Some(Ok(message)) = read.next().await {
            println!("Received message: {:?}", message);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    create_wss_client("ws://127.0.0.1:8080".to_string()).await?;
    Ok(())
}