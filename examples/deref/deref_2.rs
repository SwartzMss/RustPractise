

fn main()
{
    // 函数和方法的隐式解引用转化(deref coercion)
    // 隐式解引用转化是为函数和方法提供的一种便捷特性

    // 假设T实现了deref trait
    // Deref coercion 可以把T的引用转化为T经过Deref 操作后生成的引用 （fn deref(&self) ->&T）
    // 当把某类型的引用传递给函数或方法时，但它的类型与定义的参数类型不匹配：
    // -这时候deref coercion就会自动发生
    // -编译器会对deref进行一系列调用，来把他转换为所需的参数类型（在编译时完成，没有额外的性能开销）

    let name = Box::new(String::from("hello"));
    // &name 是 &Box<String>。
    // Rust 首先调用 Box<String> 的 deref 方法，将 &Box<String> 转换为 &String。
    // 然后，Rust 再调用 String 的 deref 方法，将 &String 转换为 &str
    hello(&name); // hello(&(*name).deref()); 
    let s = *name; //let s = *(name.deref());
    // 注意上面的两种自动解引用


    //解引用与可变性
    // 可使用DerefMut trait 重载可变引用的*运算符
    // 当类型和trait在下列三种情况发生时候， rust会指向deref coercion
    // -当T:Deref<Target=U>，允许&T转换为&U
    // -当T:DerefMut<Target=u> 允许&mut T转换为&mut U
    // -当T:Deref<Target=u> 允许&mut T转换为&U
}


fn hello(name:&str){
    println!("{}",name);
}