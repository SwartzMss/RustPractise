
fn main() {
    another_func();
    let result = test_return_value();
    println!("result {}",result);
}

fn test_return_value()->i32{

    let mut num = 0;
    loop{
        match num == 10{
            true => break 20,
            false => num = num +1,
        }
    }
}

// 函数名字的命名一般小写+下划线的方式
fn another_func()
{
    println!("another func called")
}