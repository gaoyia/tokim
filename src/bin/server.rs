use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::hash::{Hash,Hasher};
type Db = Vec<Mutex<HashMap<String, Vec<u8>>>>;
type ShardedDb = Arc<Vec<Mutex<HashMap<String, Vec<u8>>>>>;
fn new_sharded_db(num_shards: usize) -> ShardedDb {
    let mut db: Db = Vec::with_capacity(num_shards);
    for _ in 0..num_shards {
        db.push(Mutex::new(HashMap::new()));
    }
    Arc::new(db)
}
#[tokio::main]
async fn main() {
    // 绑定监听地址
    let listener: TcpListener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("Listening");

    let db:ShardedDb = new_sharded_db(5);
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        // 每个传入套接字都会生成一个新的任务。套接字被移动到新的任务并在那里进行处理。
        // 将句柄克隆到HashMap
        let db:ShardedDb = db.clone();
        println!("Accepted");

        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}


fn hash<T: Hash>(value: &T) -> usize {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish() as usize
}

async fn process(socket: TcpStream,db: ShardedDb) {
    use mini_redis::Command::{self, Get, Set};
    // mini-redis提供的 Connection 负责解析socket中的帧数据。
    let mut connection: Connection = Connection::new(socket);

    // 使用' read_frame '从连接接收命令。
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response: Frame = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let key = cmd.key().to_string();
                let value = cmd.value().clone().to_vec();
                let mut shard = db[hash(&key) % db.len()].lock().unwrap();
                println!("hash-key:{:?}",hash(&key) % db.len());
                println!("set-key:{:?}",key);
                println!("value:{:?}",value);
                shard.insert(key, value);
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let key = cmd.key().to_string();
                let shard = db[hash(&key) % db.len()].lock().unwrap();
                if let Some(value) = shard.get(cmd.key()) {
                    println!("get-key:{:?}",key);
                    println!("value:{:?}",value);
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // 为客户端编写响应
        connection.write_frame(&response).await.unwrap();
    }
}