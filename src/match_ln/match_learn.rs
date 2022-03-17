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

pub fn ref_ln() {
    let value = 5;
    match value {
        ref r => println!("Got a reference to {}", r),
    }

    let mut a = 5;

    match a {
        ref mut m => {
            *m = 6;
            println!("a is {}", m);
        }
    }
}

#[test]
pub fn ref_ln_test() {
    let value = 5;
    match value {
        ref r => println!("Got a reference to {}", r),
    }

    let mut a = "googd";
    let mut b = 5;
    match a {
        b => {
            println!("a is {}", a);
        }
        _ => {
            println!("value not equal,a is {}", a);
        }
    };

    let reference = &4;

    match reference {
        // 如果用 `&val` 这个模式去匹配 `reference`，就相当于做这样的比较：
        // `&i32`（译注：即 `reference` 的类型）
        // `&val`（译注：即用于匹配的模式）
        // ^ 我们看到，如果去掉匹配的 `&`，`i32` 应当赋给 `val`。
        // 译注：因此可用 `val` 表示被 `reference` 引用的值 4。
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    // 解构结构体的成员
    let foo = Foo { x: (1, 2), y: 3 };
    let Foo { x: (a, b), y } = foo;

    println!(" a = {}, b = {},  y = {} ", a, b, y);
}
