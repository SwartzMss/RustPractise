

fn largest<T: std::cmp::PartialOrd + Copy>(list:&[T]) ->T{
    let mut largest = list[0];
    for value in list.iter() {
        if *value > largest{
            largest = *value;
        }
    }
    largest
}

struct  Point<T:Copy>{
    x:T,
    y:T,
}
// 这边的话 也可以直接实例化类型
impl Point<i32>{

}

//把T放到impl关键字后，表示再类型T上实现方法
impl<T:Copy> Point<T>{
    fn x(&self)->T{
        self.x
    }
}

// 这的话 可以比较一下Point和PointTest,如果没实现copy的话 返回的时候就只能是引用
struct  PointTest<T>{
    x:T,
    y:T,
}
impl<T> PointTest<T>{
    fn x(&self)->&T{
        &self.x
    }
}


enum Option<T>{
    Some(T),
    None
}

enum Result<T,E>{
    Ok(T),
    Err(E)
}

fn main()
{
    // 测试 largest 函数对整数切片的调用
    let numbers = vec![10, 50, 30, 20, 100];
    let largest_number = largest(&numbers);
    println!("The largest number is: {}", largest_number);


    // 测试 largest 函数对浮点数切片的调用
    let float_numbers = vec![2.5, 3.7, 1.2, 9.8, 7.3];
    let largest_float = largest(&float_numbers);
    println!("The largest float number is: {}", largest_float);

    let p1 = Point{x : 1, y : 2};
    let p2 = Point{x : 0.2,y : 0.5};
    println!("p1.x = {}", p1.x());
}