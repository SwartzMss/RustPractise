struct  User{
    username: String,
    email: String,
    sign_in_count:u64,
    active:bool,
}



fn area(width:u32,length:u32)->u32{
    width*length
}

fn area_1(dim:(u32,u32))->u32{
    dim.0*dim.1
}
#[derive(Debug)]
struct  rectangle{
    width:u32,
    height:u32,
}

fn area_2(rect:&rectangle)->u32{
    rect.width*rect.height
}

impl rectangle {
    // 下面的两个就是方法，第一个参数是self
    fn area_3(&self) ->u32 {
        self.height *self.width
    }

    fn can_hould(&self, other:&rectangle) ->bool{
        self.height >other.height && self.width>other.width
    }

    fn square(size:u32)->Self{
        rectangle{
            width:size,
            height:size,
        }
    }
}
fn main() {

    let user1 = User{
        email:String::from("xx@qq.com"),
        username:String::from("swartz"),
        sign_in_count:2,
        active:true
    };

    // 可以简化初始化
    let user2 = User {
        email:String::from("xxx@qq.com"),
        ..user1
    };

    // tuple struct
    struct Color(i32,i32,i32);
    let black = Color(10,15,20);

    // unit-like struct 里面啥都没的结构体
}