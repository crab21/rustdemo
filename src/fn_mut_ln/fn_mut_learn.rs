use core::fmt;
use std::borrow::{Borrow, BorrowMut};

#[test]
fn test_fn_mut() {
    fn fn_immut<F>(f: F)
    where
        F: FnOnce() -> String,
    {
        println!("----------------->calling Fn closure from fn, {}", f());
    }

    let a = "Fn".to_string();
    fn_immut(|| a.clone()); // 闭包返回一个字符串
}

#[test]
fn test_fn_mut_1() {
    fn fn_immut<F>(f: F)
    where
        F: FnOnce() -> String,
    {
        println!("calling Fn closure from fn, {}", f());
    }

    let a = "Fn".to_string();
    fn_immut(|| a.clone()); // 闭包返回一个字符串
    
}

#[derive(Copy, Clone)]
struct FooCopy {
    value: i32,
}

impl FooCopy {
    fn new(value: i32) -> Self {
        Self { value }
    }
    
    fn get(&self) -> i32 {
        self.value
    }
    
    fn increase(&mut self) {
        self.value += 1;
    }
}

fn is_FnMut<F: FnMut()>(_closure: &F) {}

fn is_Copy<F: Copy>(_closure: &F) {}

#[test]
fn t_fn_closure() {
    let mut foo_copy = FooCopy::new(0);
  
    let mut c_with_move = move || {
        for _ in 0..5 {
            foo_copy.increase();
        }
        
        println!("foo_copy in closure(with move): {}", foo_copy.get());
    };
    
    c_with_move();
    println!("foo_copy out of closure: {}\n", foo_copy.get());
    

    
    // is_FnMut(&c_with_move);
    // is_Copy(&c_with_move);
    
    // is_FnMut(&c_without_move);
    //is_Copy(&c_without_move); // Error
    
    println!("foo_copy out of closure(without move): {}\n", foo_copy.get());

    let mut c_without_move =  || {
        for _ in 0..5 {
            foo_copy.increase();
        }
        
        println!("foo_copy in closure(without move): {}", foo_copy.get());
    };
    
    c_without_move();
    println!("foo_copy out of closure(without move): {}\n", foo_copy.get());
    c_with_move();
    println!("foo_copy out of closure(with move): {}\n", foo_copy.get());
}