use serde::de::Error;
use std::{borrow::{Borrow, Cow}, ops::Add, string, sync::Arc};
use List::{Cons, Nil};

#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

pub fn box_test() {
    let recursive_list: List<i32> = Cons(1, Box::new(Cons(2, Box::new(Nil))));

    println!("{:?}", recursive_list); // 打印出：Cons(1, Cons(2, Nil))
}

pub fn box_i32() {
    let a = Box::new(3);
    println!("a = {}", a); // a = 3

    // 下面一行代码将报错
    // let b = a + 1; // cannot add `{integer}` to `Box<{integer}>`
    // println!("{}",b)
}

#[test]
fn test_box_i32() {
    box_i32()
}

fn box_copy() {
    // 在栈上创建一个长度为1000的数组
    let arr = [0; 1000];
    // 将arr所有权转移arr1，由于 `arr` 分配在栈上，因此这里实际上是直接重新深拷贝了一份数据
    let arr1 = arr;

    // arr 和 arr1 都拥有各自的栈上数组，因此不会报错
    println!("{:?}", arr.len());
    println!("{:?}", arr1.len());

    // 在堆上创建一个长度为1000的数组，然后使用一个智能指针指向它
    let arr = Box::new([0; 1000]);
    // 将堆上数组的所有权转移给 arr1，由于数据在堆上，因此仅仅拷贝了智能指针的结构体，底层数据并没有被拷贝
    // 所有权顺利转移给 arr1，arr 不再拥有所有权
    let arr1 = arr;
    println!("{:?}", arr1.len());
    // 由于 arr 不再拥有底层数组的所有权，因此下面代码将报错
    // println!("{:?}", arr.len());
}

#[test]
fn test_box_copy() {
    box_copy();
}
pub fn result() -> (Result<i32, i32>) {
    let k = 21;

    let x: Result<&str, _> = Err("foo");
    println!("{}", x.map_or_else(|_e| 2, |v| 1));
    println!("{}", x.map_or_else(|_e| 2, |v| 1));
    let xx: Result<i32, i32> = Err(2);
    match xx {
        Ok(v) => println!("ok....{}", v),
        Err(v) => println!("err.....{}", v),
    }
    let a = xx?;
    Ok(a)
}

#[test]
fn test_tresult() {
    let x: Result<&str, i32> = Ok("gogo");
    let b = x.ok();
    let c = b.unwrap();
    println!("{:?} aaaaaaaaaaaaaaaaa", c.to_lowercase());
    println!("{:?}", get_result(0).unwrap());
}

fn get_result(i: i32) -> Result<Box<str>, String> {
    if i > 0 {
        return Err(String::from("err"));
    }
    // let gogo = "go";
    // Ok(gogo.borrow())
    let gogo = String::from("gogowang_");
    Ok(gogo.into_boxed_str())
}
fn get_result_str(i: i32) -> Result<Cow<'static,str>, String> {
    if i > 0 {
        return Err(String::from("err"));
    }
    // let gogo = "go";
    // Ok(gogo.borrow())
    let gogo = String::from("gogowang_");
    Ok(Cow::Owned(gogo))
}

#[test]
fn test_result() {
    let s = result();
    let b = s.map_or(8, |_| 2);
    println!("b...{}", b);


    let bow = get_result_str(0);
    let bow_wrap = bow.unwrap();
    let result = bow_wrap.into_owned();
    println!("{:?}",result)
}

#[test]
// test_store_data us
fn test_store_data() {
    let data = Box::new(5);
    println!("b = {:?}", data)
}

// Rc --------------------------------------

use std::rc::Rc;
use ListRc::{ConsRc, Nils};
#[derive(Debug)]
enum ListRc {
    ConsRc(i32, Rc<ListRc>),
    Nils,
}
#[test]
fn test_cons() {
    let a = Rc::new(ConsRc(5, Rc::new(ConsRc(2, Rc::new(Nils)))));
    let b = ConsRc(6, Rc::clone(&a));
    let c = ConsRc(7, Rc::clone(&a));
    println!("{:?} {:?} {:?}", a, b, c)
}
// Rc ---------------------------------------
