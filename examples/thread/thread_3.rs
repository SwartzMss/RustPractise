
// channel类似单所有权，一旦将值的所有权转移至channel,就无法使用了
// 共享内存的并发类似多所有权：多个线程可以同时访问同一块内存

// 使用mutex 来每次运行一个线程来访问数据
// 在使用数据前 必须尝试获得锁
// 使用完mutex所保护的数据，必须对数据进行解锁，以便其他进行可以获得锁
use std::thread;
use std::sync::Mutex;
use std::sync::Arc;
fn main(){
    // 通过Mutex::new 来创建Mutex<T> 是一个智能指针

    let m = Arc::new(Mutex::new(5));
    {
        let mut s = m.lock().unwrap();
        *s = 55;
    }

    let mut handles = vec![];

    for _ in 0..10{
        let numbers_clone = Arc::clone(&m);
        let handle = thread::spawn(move||{
            let mut num = numbers_clone.lock().unwrap();
            *num +=1;
        });
        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }
    println!("{:?}", m);
}