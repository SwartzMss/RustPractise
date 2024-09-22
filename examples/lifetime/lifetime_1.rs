
// 生命周期的必要性就是解决悬垂引用
// 生命周期的标注不会改变引用的生命周期长度
// 当指定了泛型生命周期参数，函数可以接收带有任何生命周期的引用
// 生命周期的标注： 描述了多个引用的生命周期的关系（单个生命周期没意义），但是不影响生命周期

fn longest<'a>(x:&'a str,y:&'a str) ->&'a str{
    if x.len() > y.len(){
        x
    } else{
        y
    }
}

// 从函数返回引用时，返回类型的生命周期参数需要和其中一个参数的生命周期匹配
// 如果返回的引用没指向任何参数，那么他只能引用函数内创建的值：这就是悬垂引用，该值再函数结束的时候就走出了作用域
fn longest_one<'a>(x:&'a str,y:& str) ->&'a str{
        x
}

fn main()
{
    let x = String::from("hello ");
    let y = String::from("world");

    let result = longest(&x, &y);
    println!("result = {}",result);



    // let x = String::from("hello ");
    // let result;
    // {
    //     let y = String::from("world");
    //     result = longest(&x, &y); #^^ borrowed value does not live long enough
    // }
    // println!("result = {}",result);



}