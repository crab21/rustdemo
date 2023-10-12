pub fn new_slice() {
    let s: String = String::from("hello word");
    let hello = &s[0..3];

    println!("{:}", hello);
}

pub fn new_arr() {
    let mut arr: [i32; 3] = [11, 22, 33];
    if arr.is_empty() {
        println!("{}", "arr is emtpy");
    }

    arr.reverse();
    let b = arr.as_slice();
    for i in b.into_iter() {
        println!("{}", i.to_owned())
    }
}

#[test]
fn test_new_arr() {
    new_arr()
}

#[test]
fn te() {
    let a = 0b0001_0100;
    println!("{}", a)
}
#[test]
fn index_slices() {
    let mut s = String::from("hello world");

    let hello = first_word(&s);
    // let world = &s[6..11];
    s.clear();
    println!("{}", hello);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    // s.len()
    0
}
