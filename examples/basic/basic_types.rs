

fn main() {
    let tup = (500,"123",12.01);
    println!("tup {} {} {}", tup.0, tup.1,tup.2);
    //获取元组的话
    let (x,y,z) = tup;
    println!("x={} y={} z={}", x, y,z);

    let suzhu = [1,2,3];
    println!("{:?}",suzhu);
    println!("{}",suzhu[2]);
    let suzhu = [3;4];
    println!("{:?}",suzhu);
    println!("{}",suzhu[0]);


    for num in 1..4 {
        println!("num = {}",num);
    }
    for num in (1..4).rev(){
        println!("num = {}",num);
    }
}