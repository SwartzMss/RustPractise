
// 字符串是啥？
// byte的集合
// 一些方法能将byte解析成文本
// rust的核心语言层面，只有一个字符串类型：字符串切片&str
// 字符串切片：对存储再其他地方UTF-8编码的字符串的引用


//String 类型
// 来自于标准库 而不是核心语言
// 可增长 可修改 可拥有
// UTF-8编码

// 因为String本身就是byte的集合，所以很多vec<T>的操作也都可以用于String 比如Vec::new 和String::new                                                               

fn main(){

    let mut s = String::new();
    s.push_str("abc");
    s.push('d');
    println!("s = {}", s);

    let data = "initial contents";
    let s1 = data.to_string();
    let s2 = "initial content".to_string();

    // &String 可以自动转换为 &str

    // 总结：
    // 智能指针（如 Box<T>） 不需要引用就可以进行解引用，因为它们本身是实现了 Deref 的类型，可以直接通过 * 运算符或者自动解引用机制访问内部数据。
    // 普通类型（如 String、Vec<T>） 则需要通过引用（即 & 或 &mut）才能进行解引用。这是因为它们实现了 Deref，但要触发 Deref 机制，必须通过引用来使用它们。
    s.push_str(&s1);

    let s3 = String::from("initial contents");


    let s4 = String::from("111 ");
    let s5 = String::from("222 ");
    let s6 = String::from("333 ");
    // 后面的参数都得是引用
    let s7 = s4 + &s5 + &s6;
    // println!("s4 = {}", s4); s4的所有权已经被拿走了
    println!("s5 = {}", s5);
    println!("s6 = {}", s6);
    println!("s7 = {}", s7);

}