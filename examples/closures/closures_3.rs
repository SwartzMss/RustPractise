
// 闭包可以捕获他们所在的环境（定义他的作用域内的变量，而普通函数不能）


// 与函数获得参数的三种方式一样
// 取得所有权 FnOnce
// 可变借用 fnMut
// 不可变借用 fn


// 创建闭包时，通过闭包对环境值的使用，rust推断出具体使用哪个trait
// -所有的闭包都实现了FnOnce
// -没有移动捕获变量的实现了FnMut
// -无需可变访问捕获变量的闭包实现了Fn


fn main()
{

    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));

    // move 关键字会转移所有权
    let x = vec![1,2,3];
    let equal_to_x = move|z| z== x;
    // println!("cannot use x here,{:?}",x);
    let y = vec![1,2,3];
    assert!(equal_to_x(y));
}