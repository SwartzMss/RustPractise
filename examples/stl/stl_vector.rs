
enum SpreadSheetCell{
    Int(i32),
    Float(f64),
    Text(String)
}

fn main()
{
    let v:Vec<i32> = Vec::new();

    let mut v = vec![1,2,3]; // 这边是自动推断的
    let third = &v[2]; //这边越界的话 会直接panic
    println!("&v[2] = {}",third);

    match v.get(2) {
        Some(third) =>println!("v.get(2) = {}",third),
        None => println!("no third element")
        
    }
    // 这边是必须引用的，如果不是引用的话所有权就直接转移了
    for i in & v{
        println!("i = {}",i);
    }
    println!("------------");
    for i in &mut v{
        println!("i = {}",i);
        *i = *i + 1;
    }
    // iter()是不可变引用 其实等于for i in & v
    // iter_mut  等于for i in &mut v
    // into_iter 等于for i in  v
    println!("------------");
    for i in v.iter(){
        println!("i = {}",i);
    }

    let mut v = Vec::new();
    v.push(1); // 这边也可以自动推断


    let row = vec![SpreadSheetCell::Int(3),SpreadSheetCell::Float(3.0),SpreadSheetCell::Text(String::from("blue"))];

}