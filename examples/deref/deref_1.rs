// deref trait 使得我们可以自定义解引用运算符*的行为
// 通过实现deref,智能指针可以像常规引用一样来处理
use std::ops::Deref;

// 解引用运算符
// 常规的引用也是一种指针

fn main()
{
    let x = 5;
    let y = &x;
    let z = Box::new(x);
    let a = MyBox::new(x);

    assert_eq!(x,5);
    assert_eq!(*y,5);
    assert_eq!(*z,5);
    assert_eq!(*a,5); // *(a.deref())

    // 定义自己的智能指针
    // BOX<T> 被定义成拥有一个元素的tuple struct
}

struct MyBox<T>(T);

impl<T> MyBox<T>{
    fn new(x:T)->Self{
        MyBox(x)
    }

}

impl<T> Deref for MyBox<T>{
    // 标准库种的deref trait要求我们实现一个deref方法
    // 该方法借用self 返回一个指向内部数据的引用
    type Target = T;
    fn deref(&self) ->&T{
        &self.0
    }
}