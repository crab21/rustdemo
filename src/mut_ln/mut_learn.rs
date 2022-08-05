use std::fmt;


pub fn space_ln() {
    let spaces = "   ";
    let spaces = spaces.len();
    println!("{}", spaces)
}


#[test]
fn space_ln_test(){
    space_ln()
}