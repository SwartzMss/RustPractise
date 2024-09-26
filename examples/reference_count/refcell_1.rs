
// 内部可变性是rust的设计模式之一
// 允许你在只持有不可变引用的前提下对数据进行修改
// - 数据结构中使用了unsafe代码来绕过Rust正常的可变性和借用规则
use std::cell::RefCell;

pub trait Messenger {
    fn send(&self,msg:&str);
}

pub struct LimitTracker<'a,T:'a + Messenger>{
    messenger:&'a T,
    value: usize,
    max:usize,
}


impl<'a,T> LimitTracker<'a,T>
where T:Messenger
{
    pub fn new(messenger:&T,max:usize)->LimitTracker<T>{
        LimitTracker{
            messenger,
            value:0,
            max
        }
    }

    pub fn set_value(&mut self,value:usize){
        self.value = value;
        let percentage_of_max = self.value as f64 /self.max as f64;
        if percentage_of_max >=1.0{
            self.messenger.send("Error: you are over your quota");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("urgent warning: you have used up 90% of  your quota");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("warning: you have used up 75% of  your quota");
        }
    }
}

struct MockMessenger{
    sent_messenger:RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new()->Self{
        MockMessenger{
            sent_messenger:RefCell::new(vec![])
        }
    }
}

impl Messenger for MockMessenger{
    fn send(&self,msg:&str){
        self.sent_messenger.borrow_mut().push(String::from(msg));
    }
}

fn main()
{
    // refcell<T> 与RC<T>不同，它代表了其持有数据的唯一所有权
    // box 编译阶段强制代码遵守借用规则，否则出现错误
    // refcell 只会在运行的时候进行检查，错误就直接panic 这也是单线程的
    
    let mock_msg = MockMessenger::new();
    let mut limitTracker = LimitTracker::new(&mock_msg, 100);

    limitTracker.set_value(80);
    assert_eq!(mock_msg.sent_messenger.borrow().len(),1);
}