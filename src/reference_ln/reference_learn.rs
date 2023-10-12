use std::borrow::Cow;

use serde::ser::Error;

pub fn ref_test() {
    let mut x = 5;
    // {
    let y = &mut x;
    *y += 1;
    // }
    println!("{}", x);
}

pub fn ref_borrow() {
    let y;
    let mut x = 5;
    y = &mut x;
    *y += 1;
    println!("{}", x);
    let v = vec![1, 2, 3];
    let v2 = v;
    print!("{:?}", v2);
}

#[test]
fn t_dangle() {
    let reference_to_nothing = dangle();
}

fn dangle() ->  Cow<'static ,str>{  // dangle 返回一个字符串的引用
    let s:  String = String::from("hello");  // s 是一个新字符串
    Cow::Owned(s)
    
    // rs.to_owned().as_ptr().to_owned()
}// 这里 s 离开作用域并被丢弃。其内存被释放。

#[test]
fn ttmain(){
    let mut x = String::from("hello");

    // x将所有权转移给y，但y无法修改字符串
    let mut y = x;   
    y.push('C');  // 本行报错
    
    let a = String::from("hello");
    // 虽然a无法修改字符串，但转移所有权后，b可修改字符串
    let mut b = a; 
    b.push('C');   // 本行不报错
  }
  