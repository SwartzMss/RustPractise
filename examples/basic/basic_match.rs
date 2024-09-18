enum  Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin:Coin) ->u8{
    match coin{
        Coin::Penny => 1,
        Coin::Nickel =>5,
        Coin::Dime =>10,
        Coin::Quarter(state) => 25,
    }
}

#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
}


fn main()
{
    let coin_1 = Coin::Penny;
    let value = value_in_cents(coin_1);

    let coin_2 = Coin::Quarter(UsState::Alaska);
    let value = value_in_cents(coin_2);

    let v= 1u8;
    match v{
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        _ => println!("{}" , v),
    }

    if let 10 = v {
        println!("10");
    }
}