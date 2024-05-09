#[derive(Debug)]
enum UsState {
    Alas,
    NewYork
}

enum Coin {
    Penny,
    Nickel,
    Quarter(UsState)
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            10
        }
    }
}

fn main() {
    // 从enum变体中提取值
    let c = Coin::Quarter(UsState::NewYork);
    println!("{}", value_in_cents(c))
}
