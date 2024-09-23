

// 所有的迭代器都实现了iterator trait
// fn next(&mut self) -> Option<Self::Item>;->实现iterator 需要你定义一个item类型，它用于next方法的返回类型
// next ：每次返回迭代器中的一项 返回结果包括在Some里 迭代结束，返回None
// 可直接在迭代器上调用next方法 
fn main()
{
    let v1 = vec![1,2,3];
    let mut v1_iter = v1.iter();    

    assert_eq!(v1_iter.next(),Some(&1));
    assert_eq!(v1_iter.next(),Some(&2));
    assert_eq!(v1_iter.next(),Some(&3));
    assert_eq!(v1_iter.next(),None);


    // 几个迭代器方法
    // iter 在不可变引用上创建迭代器
    // into_iter 创建的迭代器会获得所有权
    // iter_mut 迭代可变引用

    // 消耗迭代器的方法 调用next的方法叫做 消耗型适配器
    // 因为调用他们会把迭代器耗尽 例如 sum方法 （取得迭代器的所有权，通过反复调用next 遍历所有元素，每次迭代把当前元素添加到总和 迭代结束 返回总和）
    let sum:i32 = v1.iter().sum();
    println!("sum = {}", sum);

    //迭代器适配器  换句话说 也就是把迭代器转换为不同种类的迭代器的方法
    // 例如 map ->接收一个闭包 闭包作用于每个元素转换为另一组元素
    // collect 是消耗型适配器，他会把结果收集到一个集合类型里面
    let v2:Vec<_> = v1.iter().map(|x| x+1).collect(); // v2 的类型是 Vec<i32> 因为map会生成一个新的元素
    let v3 = v1;
    println!("v2 = {:?}",v2);


}