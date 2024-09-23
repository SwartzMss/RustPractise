// 使用iterator trait来创建自定义的迭代器 ，其实也就是要实现next方法

struct Counter{
    count:u32,
}

impl Counter{
    fn new()->Self{
        Counter{count:0}
    }
}

impl Iterator for Counter{
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count<5{
            self.count += 1;
            Some(self.count)
        } else{
            None
        }
    }
}

fn main()
{
    let  cc1 = Counter::new();

    for it in cc1.into_iter(){
        println!("{}",it);
    }

    let mut cc2 = Counter::new();

    assert_eq!(cc2.next(),Some(1));
    assert_eq!(cc2.next(),Some(2));
    assert_eq!(cc2.next(),Some(3));
    assert_eq!(cc2.next(),Some(4));
    assert_eq!(cc2.next(),Some(5));
    assert_eq!(cc2.next(),None);
}