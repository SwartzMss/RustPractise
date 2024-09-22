use std::{fmt::Debug, fmt::Display};

pub trait  Summary {
    // 默认实现也可以调用虚函数
    fn summarize(&self)->String;
}

pub struct NewsArticle{
    pub headline:String,
    pub location:String,
    pub author:String,
    pub content:String,
}

impl Summary for NewsArticle {
    fn summarize(&self)->String {
        format!("{}, by {} ({})", self.headline,self.author,self.location)
    }
}

#[derive(Debug)]
pub struct Tweet{
    pub username:String,
    pub content:String,
    pub reply:bool,
    pub reweet:bool,
}

impl Summary for Tweet{
    fn summarize(&self)->String {
        format!("{}: {}", self.username,self.content)
    }

}

// trait 作为参数
pub fn notify(item: impl Summary) {
    println!("{}", item.summarize());
}

// trailt bound 
pub fn notify_ex<T:Summary>(item: T){
    println!("{}", item.summarize());
}


pub fn notify_ex_1<T:Summary+Display,U:Clone + Debug>(item1: T,item2:U){
    println!("{}", item1.summarize());
}

pub fn notify_ex_2<T,U>(item1: T,item2:U)
where T:Summary+Display,
      U:Clone + Debug
{
    println!("{}", item1.summarize());
}

// 使用trait 作为返回值
pub fn notify_news() ->impl Summary{
    NewsArticle{
        headline:String::from("hello"),
        author: String::from("Swartz"),
        location:String::from("china"),
        content:String::from("content"),
    }
}
// 有可能返回不同的类型
pub fn notify_by_flag(flag: bool) -> Box<dyn Summary> {
    if flag {
        Box::new(NewsArticle {
            headline: String::from("Example headline"),
            author: String::from("John Doe"),
            location: String::from("New York"),
            content: String::from("Here is some content."),
        })
    } else {
        Box::new(Tweet {
            username: String::from("@example"),
            content: String::from("Example tweet content"),
            reply: false,
            reweet: false,
        })
    }
}

fn main(){
    let news = NewsArticle{
        headline:String::from("hello"),
        author: String::from("Swartz"),
        location:String::from("china"),
        content:String::from("content"),
    };

    let tw = Tweet{
        username:String::from("aaron"),
        content:String::from("content"),
        reply:false,
        reweet:false
    };

    notify(news);
    
    notify_ex(tw);


    // 调用 `notify_by_flag` 函数并传入 true 或 false 来获取不同的实现
    let article_summary = notify_by_flag(true);
    let tweet_summary = notify_by_flag(false);

    // 打印每个对象的摘要
    println!("News Article Summary: {}", article_summary.summarize());
    println!("Tweet Summary: {}", tweet_summary.summarize());
}