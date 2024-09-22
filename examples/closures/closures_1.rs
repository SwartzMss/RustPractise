// 闭包就是可以捕获其所在环境的匿名函数

use std::{hash::RandomState, thread, time::Duration};

// 例子 生成自定义运行计划的程序
// 算法的逻辑不是重点，重点是算法中的计算过程需要几秒钟时间
// 目标：不让用户发生不必要的等待
//    -- 仅在必要时调用该算法
//    -- 只调用一次
fn main()
{
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value,simulated_random_number);

}



fn generate_workout(intensity:u32,random_number:u32){

    // 不强制要求标注参数和返回值类型
    // 闭包的定义最终只会为参数、返回值推断出唯一具体的类型  换句话说  不能给一个闭包函数传递不同类型的参数，他不是模板
    let expensive_closure = |intensity:u32|{
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity
    };
    // let result = simulate_expensive_calculation(intensity);
    if intensity < 25{
        println!(
            "Today, do {} pushups",
            expensive_closure(intensity)
        );
        println!(
            "next, do {} situps",
            expensive_closure(intensity)
        );
    } else{
        if random_number == 3 {
            println!("take a break today! remember to stay hydrated!")
        } else {
            println!(
                "Today, run for {} minutes",
                expensive_closure(intensity)
            );
        }
    }
}


fn simulate_expensive_calculation(intensity:u32)->u32{
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}