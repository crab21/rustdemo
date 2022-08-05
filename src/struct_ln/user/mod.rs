pub mod fsss;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

pub fn use_user() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("{}", user1.email);
    fsss::bbbbb();


    println!("{}", user1.username);

    user1.email = String::from("another email");

    // let result = json::stringify(&user1);

    // println!("{}", result);
    println!("gooooooooooooooooooooooooooooooo");
    let re = serde_json::to_string(&user1).unwrap();
    println!("===============>{}", re);
}
