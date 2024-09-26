
//RC 是单线程使用，不在预导入模块
//Rc::Clone(&a) 增加引用计数
//Rc::strong_count(&a) 获得引用计数
//Rc::weak_count(&a) 获得弱引用计数
use std::rc::Rc;
enum List{
    Cons(i32, Rc<List>),
    Nil
}
use crate::List::{Cons,Nil};

fn main(){
    let a = Rc::new(Cons(5, 
        Rc::new(Cons(10, 
            Rc::new(Nil)))));
    
    println!("strong_count {}",Rc::strong_count(&a));

    // Rc::clone 增加引用，不会执行数据的深度拷贝
    // RC<T>是不可变引用，使得可以在程序的不同部分之间共享只读数据
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    println!("strong_count {}",Rc::strong_count(&a));
}