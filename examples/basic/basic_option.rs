

fn main() {

    let  some_number = Some(5);
    let some_string = Some("String");

    let absent_number:Option<i32> = None;

    // option<i32> + i32 会编译错误，必须进行类型转换
    let opt = Some(5);
    let num = 10;

    let result = match opt {
        Some(value) => value + num,  // 如果是 Some，进行相加
        None => num,                 // 如果是 None，返回原数
    };
}