use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // 创建一个 Arc 包裹的 Mutex
    let counter = Arc::new(Mutex::new(0));
    
    // 创建多个线程
    let mut handles = vec![];

    for _ in 0..10 {
        // 克隆 Arc，以便在不同线程中共享所有权
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    // 输出最终的计数值
    println!("Result: {}", *counter.lock().unwrap());
}
