// 关于derive的解释：https://rustwiki.org/zh-CN/rust-by-example/trait/derive.html
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

pub enum Coin {
    Penny,
    Dime,
    Good,
    Quarter(UsState),
}
pub fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Dime => {
            println!("Dime!");
            10
        }
        Coin::Good => {
            println!("Good!");
            100
        }
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
