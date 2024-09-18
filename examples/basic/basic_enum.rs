enum IpAddrKind {
    IPV4,
    IPV6
}

struct IpAddr{
    kind:IpAddrKind,
    address:String,
}

enum IpAddrKind1 {
    IPV4(u8,u8,u8,u8),
    IPV6(String),
}

enum Message{
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

impl Message{
    fn call(&self){
    }
}


fn main() {

    let four = IpAddrKind::IPV4;
    let six = IpAddrKind::IPV6;
    route(four);
    route(six);

    let home = IpAddr{
        kind:IpAddrKind::IPV4,
        address:String::from("127.0.0.1"),
    };

    let loopback = IpAddr{
        kind:IpAddrKind::IPV6,
        address:String::from("::1"),
    };

    let home = IpAddrKind1::IPV4(127, 0, 0, 1);
    let loopback = IpAddrKind1::IPV6(String::from("::1"));

    let q = Message::Quit;
    let m = Message::Move { x: 12, y: 12 };
    let w = Message::Write(String::from("go ahead"));
    let C = Message::ChangeColor(45, 21, 31);
}


fn route(Ip:IpAddrKind){

}