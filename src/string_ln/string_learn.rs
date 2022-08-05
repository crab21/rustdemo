// https://doc.rust-lang.org/book/ch08-02-strings.html

use std::mem::size_of;

#[test]
fn test_new() {
    // str_new();
    // bigger_test();
    // hello_world();
    split_string();
}

pub fn new() {
    append1();

    append2();

    append3();

    string_to();
    str_new();
}

fn append1() {
    let mut s = String::new();
    let s1 = String::from("hello");
    let s2: &str = "hello";
    let s3 = s + s1.as_str();
    println!("{}", s3);
}

fn append2() {
    let mut s = String::new();
    s.push_str("good");
    println!("{}", s);
}
fn append3() {
    let mut s = String::from("hello-----------");
    let mut s1 = String::from("world");
    for c in s1.chars() {
        s.push(c);
    }

    println!("{}", s);
}

fn string_to() {
    let s = String::from("👉🏻");

    let result = s.encode_utf16().collect::<Vec<u16>>();
    println!("{:?}", result);

    let a = String::from_utf16(&result).unwrap();
    println!("{}", a);
}

fn str_new() {
    let mut a = String::new();
    a.push_str("王培源");
    let capa = a.capacity();

    println!("{}", capa);

    let b = a.as_bytes();
    println!("{}", b.len());

    let c: char = 'a';
    println!("{}", c.len_utf16());

    println!("{}", size_of::<i32>())
}

fn bigger<'s>(str1: &'s str, str2: &'s str) -> &'s str {
    if str1 > str2 {
        str1
    } else {
        str2
    }
}
#[test]
fn bigger_test() {
    println!("{}", bigger("a", "b"));
}

fn hello_world<'s>() -> &'s str {
    return "Hello World!";
}


struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn split_string() {
    let novel = String::from("Call me Ishmael Some years ago");
    let mut  a = novel.split('.');
    // for i in a.next() {
    //     println!("{}==========>",i);    
    // }
    let s = a.next();
    for abs in s.iter(){
        println!("{}==========>",abs);    
    }

    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
