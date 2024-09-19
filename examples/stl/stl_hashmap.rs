use std::collections::HashMap;

fn main(){
    let mut socres = HashMap::new();
    socres.insert(String::from("blue"), 10);
    socres.insert(String::from("yellow"), 20);

    // 另一种创建hashmap的方法 collect 
    // 在 Rust 中，HashMap 必须显式指定，但其中的键和值的类型可以通过 类型推导 自动推断
    let teams =vec![String::from("blue"),String::from("yellow")];
    let init_socres = vec![10,15];
    let score_ext:HashMap<_,_> = teams.into_iter().zip(init_socres.iter()).collect();
    println!("score_ext = {:#?}",score_ext);

    // 当 HashMap 的键类型是 String 时，你可以同时使用 &str 和 &String 来进行查找。这是因为 String 实现了 Borrow<str>，
    // 而 &String 通过自动解引用也能转换为 &str，因此这两种类型在查找时都能匹配 HashMap<String, _> 的键类型
    match score_ext.get("blue"){
        Some(value) =>println!("value = {}",value),
        None =>println!("not found")
    }

    match score_ext.get(&"blue".to_string()){
        Some(value) =>println!("value = {}",value),
        None =>println!("not found")
    }


    // 使用 &String 进行查找，匹配 HashMap<&String, _> 的键类型。
    let mut socres = HashMap::new();
    let blus = String::from("blue");
    let yellow = String::from("yellow");
    socres.insert(&blus, 10);
    socres.insert(&yellow, 20);

    match socres.get(&"blue".to_string()){
        Some(value) =>println!("value = {}",value),
        None =>println!("not found")
    }

    for(k,v) in socres.iter(){
        println!("k = {} ,v = {}",k,v);
    }


    // 更新hashmap 
    // 1. k已经存在 对应一个V
    //  1.1 替换现有的V
    //  1.2 保留现有的V 忽略新的V
    //  1.3 合并现有的V和新的V
    // 2. k不存在的话 就直接新增一个即可

    // 1.1 替换现有的V
    let mut socres = HashMap::new();
    let blus = String::from("blue");
    let yellow = String::from("yellow");
    socres.insert(&blus, 10);
    socres.insert(&blus, 20);
    println!("socres = {:#?}",socres);

    //  1.2 保留现有的V 忽略新的V
    // 这边的话 也可以看看or_insert的返回值，返回的是一个可变引用
    let xx = socres.entry(&blus).or_insert(66);
    println!("xx = {:#?}",xx);
    *xx = *xx + 10; 
    println!("socres = {:#?}",socres);
    let yy = socres.entry(&yellow).or_insert(77);
    println!("socres = {:#?}",socres);

    

}