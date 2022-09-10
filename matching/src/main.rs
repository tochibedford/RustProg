enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin:: Quarter => 25
    }
}

fn main() {
    let _coin = Coin::Penny;
    let _coin = Coin::Nickel;
    let _coin = Coin::Dime;
    let coin = Coin::Quarter;

    dbg!(value_in_cents(coin));
}
