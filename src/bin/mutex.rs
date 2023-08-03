use std::sync::{Mutex, Arc};

fn main() {
    // 创建一个 Mutex 包装的共享数据
    let data = Arc::new(Mutex::new(42));

    // 在闭包中获取 MutexGuard 并操作数据
    {
        let mut guard = data.lock().unwrap();
        *guard += 1;  // 修改共享数据
        println!("Data: {}", *guard);  // 读取共享数据
        // 在 guard 退出作用域时自动释放锁
    }
    
    // 在另一个线程中获取 MutexGuard 并操作数据
    std::thread::spawn({
        let data = Arc::clone(&data);
        move || {
            let mut guard = data.lock().unwrap();
            *guard *= 2;  // 修改共享数据
            println!("Data: {}", *guard);  // 读取共享数据
            // 在 guard 退出作用域时自动释放锁
        }
    }).join().unwrap();
}