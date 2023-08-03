use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    // 创建一个被锁定的共享数据结构
    let data = Arc::new(Mutex::new(0));

    // 创建多个异步任务，对共享数据进行操作
    let tasks = (0..5).map(|i| {
        let data = Arc::clone(&data);
        tokio::spawn(async move {
            // 获取互斥锁并操作共享数据
            let mut guard = data.lock().await;
            *guard += i;
            println!("Updated data: {}", *guard);
            // 在 guard 退出作用域时自动释放锁
        })
    });

    // 等待所有异步任务完成
    for task in tasks {
        task.await.unwrap();
    }

    // 在主任务中访问共享数据
    let guard = data.lock().await;
    println!("Final data: {}", *guard);
}