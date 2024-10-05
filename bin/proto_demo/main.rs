
#[path = "tutorial.rs"]
pub mod person;


fn main()
{
    let person = person::Person {
        name: "Alice".to_string(),
        id: 123,
        email: "alice@example.com".to_string(),
        special_fields: ::protobuf::SpecialFields::new(), // 创建SpecialFields实例
    };
    
    // 使用person实例
    println!("person: {}", person);
}