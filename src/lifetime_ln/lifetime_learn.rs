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


#[test]
fn test_failed_borrow(){
   let result =  success_borrow();
   println!("{:?}",result)
}
fn success_borrow<'a>() ->&'a i32{
    let _x: &'a i32 = &(12);

    let y: &'a i32 = &_x;
    y
}

// A function which takes no arguments, but has a lifetime parameter `'a`.
fn failed_borrow<'a>() {
    let _x = 12;

    // ERROR: `_x` does not live long enough
    // let y: &'a i32 = &_x;
    // Attempting to use the lifetime `'a` as an explicit type annotation 
    // inside the function will fail because the lifetime of `&_x` is shorter
    // than that of `y`. A short lifetime cannot be coerced into a longer one.
}