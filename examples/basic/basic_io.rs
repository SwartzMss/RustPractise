use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("猜数!");
    println!("猜测一个数");

    let secret_num = rand::thread_rng().gen_range(1, 101);


    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("读取失败");
        // 使用parse的话 就需要显示的去指定转后的类型
        let guess_i32:i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {println!("非数字，继续输入");continue;}  
        };
        match guess_i32.cmp(&secret_num) {
            Ordering::Less =>println!(" too small"),
            Ordering::Greater => println!("too large"), 
            Ordering::Equal => {println!(" you win");break;}    
        }
    }

}