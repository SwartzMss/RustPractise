fn main() {

    let s = String::from("hello world");
    take_ownership(s);

    let x = 5;
    make_copy(x);
    println!("x = {}",x);

    println!("{}", give_ownership());

    let s = String::from("hello world");
    println!("{}", take_and_giveback_ownership(s));

    let mut s = String::from("hello world");
    {
        let _ss = &s; 
        let _ssa = &mut s;
        _ssa.push_str("string");
        // println!("{}", _ss);  放开的话 会有报错 因为可变引用和不可变引用不能同时存在。 但是需要检测是否后面会用到。
    }
    let len = calculate_length(&mut s);
    println!("len = {}", len);
}

fn take_ownership(some_string:String){
    println!("{}", some_string);
}
fn take_and_giveback_ownership(some_string:String) ->String{
    println!("{}", some_string);
    some_string
}

fn give_ownership()->String{
    let aa = String::from(" give hello world");
    aa
}

fn make_copy(some_number:i32){
    println!("{}",some_number);
}

fn calculate_length(s:&mut String) ->usize{
    s.push_str("string");
    s.len()
}