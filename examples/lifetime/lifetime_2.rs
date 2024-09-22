
// 结构体定义的中的生命周期标注

use std::fmt::Display;

struct A<'a >{
    part: &'a str
}

// 生命周期省略的三个规则
//1.应用于输入生命周期，每个引用类型的参数都有自己的生命周期
//2.如果只有一个输入生命周期参数，那么该生命周期被赋予所有的输出生命周期参数
//3.如果有多个输入生命周期参数，但其中一个是&self或者&mut self 那么self的生命周期会被赋予所有的输出生命周期参数
impl<'a> A<'a>{
    fn level(&self)->i32{
        3
    }

    fn cc(&self,text:&str) ->&str{
        &self.part
    }
}

//静态生命周期 'static 是一个特殊的生命周期，整个程序的持续时间
// 所有的字符串字面值都有'static 生命周期 let s:&'static str = "hello world";


fn longest_announcement<'a,T>(x:&'a str, y:&'a str, ann:T)->&'a str
where T:Display,
{
    println!("announcement {}", ann);
    if x.len() > y.len(){
        x
    } else {
        y
    }
}



fn main(){
    let s:&'static str = "hello world";
    let movel = String::from("value");
    let aA = A{part:&movel};

}