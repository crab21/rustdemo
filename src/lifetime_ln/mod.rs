use crate::lifetime_ln::lifetime_learn::lifetime_test;

mod lifetime_learn;
mod bounds;
pub fn start() {
    println!("lifetime_learn.............start");

    lifetime_test()
}
