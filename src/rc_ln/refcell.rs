use std::fmt;

#[test]
fn space_ln_test(){
    space_ln()
}

#[derive(PartialEq)]
struct Inches(i32);

use std::cell::RefCell;

pub fn space_ln() {
    // 创建一个包含整型值的 `RefCell`
    let data = RefCell::new(5);

    // 获取一个不可变借用
    {
        let borrowed_data = data.borrow();
        println!("Borrowed data: {}", borrowed_data);
    } // `borrowed_data` 在此处超出作用域，释放借用

    // 获取一个可变借用
    {
        let mut mutable_borrow = data.borrow_mut();
        *mutable_borrow += 10; // 修改数据
        println!("Mutated data: {}", mutable_borrow);
    } // `mutable_borrow` 在此处超出作用域，释放借用

    // 通过 `RefCell` 的 `borrow` 方法再次获取不可变借用
    let borrowed_data_again = data.borrow();
    println!("Data after mutation: {}", borrowed_data_again);
}