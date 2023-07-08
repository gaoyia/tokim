use bytes::Bytes;
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use mini_redis::client;

/// Provided by the requester and used by the manager task to send
/// the command response back to the requester.
type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    }
}


#[tokio::main]
async fn main() {
    // 创建一个多生产者、单消费者队列通道
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();//克隆出另一个生产者

    // 生成两个任务，一个获取key，另一个设置key
    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: "foo".to_string(),
            resp: resp_tx,
        };

        // Send the GET request
        if tx.send(cmd).await.is_err() {
            eprintln!("connection task shutdown");
            return;
        }

        // Await the response
        let res = resp_rx.await;
        println!("GOT (Get) = {:?}", res);
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
            resp: resp_tx,
        };

        // Send the SET request
        if tx2.send(cmd).await.is_err() {
            eprintln!("connection task shutdown");
            return;
        }

        // Await the response
        let res = resp_rx.await;
        println!("GOT (Set) = {:?}", res);
    });

    
    // 使用 `move` 关键字  将`rx`的所有权移动到任务中.
    let manager = tokio::spawn(async move {
        // 建立到服务器的连接
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        // 开始接收消息
        while let Some(cmd) = rx.recv().await {
            use Command::*;

            match cmd {
                Get {resp, key } => {
                    let res = client.get(&key).await;
                    // Ignore errors
                    let _ = resp.send(res);
                }
                Set { key, val,resp } => {
                    let res = client.set(&key, val).await;
                    // Ignore errors
                    let _ = resp.send(res);
                }
            }
        }
    });


    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();


}