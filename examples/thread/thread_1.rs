

use std::thread;
use std::time::Duration;

fn main(){
    // thread::spawn 函数的返回类型是JoinHandle 并且持有值的所有权
    let handle = thread::spawn(||{
        for i in 1..10{
            println!("hi num {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5  {
        println!("hi num {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 通过join handle来等待所有的线程执行结束
    // 会阻止当前运行线程的执行，直到handle所表示的这些线程终结
    handle.join().unwrap();
}