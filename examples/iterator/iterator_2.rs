#[derive(Debug,PartialEq)]
struct Shoe{
    size:u32,
    style:u32,
}


fn shoes_in_my_size(shoe:Vec<Shoe>,shoe_size:u32) ->Vec<Shoe>{
    // filter 和map是不一样的 map会生成新的元素，但是filter的话 其实默认返回的是引用，所以这边就必须转移所有权
    shoe.into_iter().filter(|x| x.size == shoe_size).collect()
}
fn main()
{
    // 使用闭包捕获环境
    // filter方法 ->接受一个闭包->这闭包在遍历迭代器的每个元素时，返回bool类型-> 如果闭包返回true:当前元素将会包含在filter产生的迭代器中
    let shoes = vec![
        Shoe { size: 10, style: 1 },
        Shoe { size: 13, style: 2 },
        Shoe { size: 10, style: 3 },
    ];

    let my_size_shoes = shoes_in_my_size(shoes, 10);
    println!("{:?}", my_size_shoes);

}