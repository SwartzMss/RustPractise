fn main() {
    let str1 = String::from("hello world");
    let num = first_word(&str1);
    println!("num = {}", num);

    let gg = [1,2,3];

    // 下面可以看出引用和切片引用的区别 
    let asa = &gg;
    let asasa = &gg[..];

    let hello =&str1[0..5];
    let world = &str1[6..];

    let xx = first_word_ex(&str1);
    println!("xx = {}",xx);
    let xx = first_word_ex2(&str1[..]);
    println!("xx = {}",xx);
    let xx = first_word_ex2("hello world");
    println!("xx = {}",xx);
}

fn first_word(s:&String) ->usize{
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate() {
        if item == b' '{
            return i;
        }
    }
    bytes.len()
}


fn first_word_ex(s:&String) ->&str{
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}

fn first_word_ex2(s:&str) ->&str{
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}