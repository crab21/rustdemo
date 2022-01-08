use std::borrow::BorrowMut;

fn print_lifetime<'a>(s1: &str, s2: &'a str) -> &'a str {
    println!("{}", s1);
    s2
}

pub fn lifetime_test() {
    let some_str: String = "Some string".to_string();
    let other_str: String = "Other string".to_string();
    let mut s1 = print_lifetime(&some_str, &other_str);
    let s3 = String::from("good info").to_string();
    let mut ss1 = String::from("good info").to_string();
    s1 = ss1.borrow_mut();
    println!("{}", s1);
}
