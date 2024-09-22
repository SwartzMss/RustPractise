pub trait  Summary {
    // 默认实现也可以调用虚函数
    fn summarize(&self)->String
    {
        format!("default implement {}",self.summarize_author())
    }

    fn summarize_author(&self) ->String;
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
    fn summarize_author(&self)->String {
        format!("{}, by {} ({})", self.headline,self.author,self.location)
    }
}

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

    fn summarize_author(&self)->String {
        format!("{}: {}", self.username,self.content)
    }

}

fn main(){
    let news = NewsArticle{
        headline:String::from("hello"),
        author: String::from("Swartz"),
        location:String::from("china"),
        content:String::from("content"),
    };
    let s = news.summarize();
    println!("news = {}" , &s);

    let tw = Tweet{
        username:String::from("aaron"),
        content:String::from("content"),
        reply:false,
        reweet:false
    };
    let s1 = tw.summarize();
    println!("tw = {}" , &s1);
}