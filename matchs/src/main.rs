enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let coin = Coin::Penny; // enum value
    let value = value_in_cents(coin);
    println!("Value in cents is {}", value);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
