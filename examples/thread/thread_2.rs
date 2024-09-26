use std::thread;
use std::sync::mpsc;


// mpsc 多个生产者 一个消费者 返回一个元组 分别是发送方 和接收方

fn main()
{
     
     let (tx,rx) = mpsc::channel();

     let tx1 = mpsc::Sender::clone(&tx);
     thread::spawn(move||{
        let val = String::from("hi");
        // 如果有问题（比如接收端已经被丢弃）就会返回一个错误
        tx.send(val).unwrap();
        // println!("{}", val); 发送之后 所有权就已经转移了
     });

     thread::spawn(move||{
        let val = String::from("tom");
        // 如果有问题（比如接收端已经被丢弃）就会返回一个错误
        tx1.send(val).unwrap();
        // println!("{}", val); 发送之后 所有权就已经转移了
     });

    // 阻塞当前线程的执行 直到有值进来
    // 当发送端关闭也一样会返回一个错误
    // try_receive 不会阻塞，需要循环调用来检查
    let ret = rx.recv().unwrap();
    println!("receive {}", ret);

    // for 循环的话也可以直接把他当成迭代器了
    for rece in rx{
        println!("rece {}", rece);
    }
}