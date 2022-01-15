use std::borrow::{Borrow, BorrowMut};
use std::str::Chars;

pub fn test_for_range() {
    let mut x = 0;
    for i in 0..10 {
        x += i;
    }
    assert_eq!(x, 45);
    println!("{}", x);
}

pub fn test_for_vec() {
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    println!("{:?}", names);
}

pub fn test_for_vec_mut() {
    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "good",
            _ => "bad",
        }
    }
    println!("names: ==> {:?}", names);
}

pub fn test_for_tuple_destructuring() {
    let triple = (0, -2, 3);
    // TODO ^ Try different values for `triple`

    println!("Tell me about {:?}", triple);
    // Match can be used to destructure a tuple
    match triple {
        // Destructure the second and third elements
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..) => println!("First is `1` and the rest doesn't matter"),
        // `..` can be used to ignore the rest of the tuple
        _ => println!("It doesn't matter what they are"),
        // `_` means don't bind the value to a variable
    }
}

pub fn test_for_guard() {
    let pair = (2, -2);
    println!("Tell me about {:?}", pair);

    match pair {
        (x, y) if x == y => println!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
}

pub fn test_for_guard_single() {
    let pair = 3;
    match pair {
        x if x % 2 == 0 => println!("The number is even!"),
        _ => println!("The number is odd!"),
    }
    println!("{}", pair);
}

fn age() -> u32 {
    15
}

pub fn test_for_guard_binding() {
    println!("Tell me what type of person you are");

    match age() {
        0 => println!("I haven't celebrated my first birthday yet"),
        // Could `match` 1 ..= 12 directly but then what age
        // would the child be? Instead, bind to `n` for the
        // sequence of 1 ..= 12. Now the age can be reported.
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        // Nothing bound. Return the result.
        n => println!("I'm an old person of age {:?}", n),
    }
}

fn some_number() -> Option<u32> {
    Some(42)
}
pub fn test_for_guard_binding_single() {
    match some_number() {
        Some(n) => println!("Not interesting... {}", n),
        Some(n @ 42) => println!("The number is {}", n),
        // Match anything else (`None` variant).
        _ => (),
    }

    let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];

    // pick the first item:
    let first = fruits.get(0);
    println!("{:?}", first);
}

#[test]
fn test_vec() {
    // let v = vec![];
    // let mut f = v.len();
    // let j: isize = 20;
    // while 0 < j {
    //     f += 1;
    // }
    // let jj: isize = 20;
    // let vv = v[j as usize];
    // println!("{}", vv);
}

#[test]
pub fn longest_test() {
    let s: String = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".to_string();
    Solution::longest_palindrome(s);
}

struct Solution {}
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() == 0 {
            return "".to_string();
        }
        let (mut start, mut end) = (0, 0);
        let b = s.as_bytes();
        let mut maxlen = -1;
        let slen = s.len() as i32;
        for i in 0..slen as i32 {
            let mut tmplen = -1;
            let mut j: i32 = -1;
            loop {
                if i - j - 1 >= 0
                    && i + j + 1 < slen
                    && b[(i - j - 1) as usize] == b[(i + j + 1) as usize]
                {
                    j += 1;
                    tmplen += 2;
                } else {
                    if maxlen < tmplen {
                        maxlen = tmplen;
                        start = i - j;
                        end = i + j;
                    }
                    break;
                }
            }

            tmplen = 0;
            j = -1;
            loop {
                if i - j - 1 >= 0
                    && i + 2 + j < slen
                    && b[(i - j - 1) as usize] == b[(i + 2 + j) as usize]
                {
                    j += 1;
                    tmplen += 2;
                } else {
                    if maxlen < tmplen {
                        maxlen = tmplen;
                        start = i - j;
                        end = i + j + 1;
                    }
                    break;
                }
            }
        }
        (s[start as usize..=end as usize]).to_string()
    }
}
