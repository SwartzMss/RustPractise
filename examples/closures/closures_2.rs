
use std::{hash::RandomState, thread, time::Duration};
// 如何让struct 持有闭包
// struct的定义需要知道所有字段的类型
// -需要显示的指明闭包的类型
// 每个闭包实例都有自己唯一的匿名类型，即使两个闭包的签名完全一样
// 所以需要使用：泛型和trait bound



// 所有的闭包都至少实现了以下trait之一:
// fn
// fnMut
// fnOnce


struct Cacher<T>
    where T: Fn(u32)->u32
{
    calculation:T,
    value: Option<u32>
}

impl <T> Cacher<T>
where T:Fn(u32)->u32
{
    fn new(calculation:T)->Self{
        Cacher{
            calculation:calculation,
            value:None
        }
    }

    fn value(&mut self, arg:u32) ->u32{
        match self.value{
            None => {let v = (self.calculation)(arg);self.value = Some(v);v},
            Some(value) => value,
        }
    }
}

fn main()
{
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value,simulated_random_number);
}

fn generate_workout(intensity:u32,random_number:u32){


    let mut expensive_closure = Cacher::new(|num|{
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    // let result = simulate_expensive_calculation(intensity);
    if intensity < 25{
        println!(
            "Today, do {} pushups",
            expensive_closure.value(intensity)
        );
        println!(
            "next, do {} situps",
            expensive_closure.value(intensity)
        );
    } else{
        if random_number == 3 {
            println!("take a break today! remember to stay hydrated!")
        } else {
            println!(
                "Today, run for {} minutes",
                expensive_closure.value(intensity)
            );
        }
    }
}