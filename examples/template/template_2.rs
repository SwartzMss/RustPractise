
struct  Point<T,U>{
    x:T,
    y:U,
}

impl <T:Copy,U> Point<T,U>{
    fn mixup<V,W>(&self,other:Point<V,W>)->Point<T,W>{
        Point{
            x: self.x,
            y: other.y,
        }
    }
}
// 还是需要注意一下self  和&self
impl <T,U> Point<T,U>{
    fn mixupEx<V,W>(self,other:Point<V,W>)->Point<T,W>{
        Point{
            x: self.x,
            y: other.y,
        }
    }
}
fn main()
{
    let p1 = Point{x : 1, y : 2};
    let p2 = Point{x : "hello",y : 'c'};
    let p3 = p1.mixup(p2);
    let p4 = Point{x : "hello",y : 'c'};
    let p5 = p1.mixupEx(p4);

    println!("p3.x = {},p3.y = {}",p3.x, p3.y);
    println!("p5.x = {},p5.y = {}",p5.x, p5.y);
}