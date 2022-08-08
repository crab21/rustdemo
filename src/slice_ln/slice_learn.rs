


pub fn new_slice(){
    let s: String = String::from("hello word");
    let hello  = &s[0..3];

    println!("{:}", hello);
}

pub fn new_arr(){
    let mut arr :[i32;3] = [11,22,33];
    if arr.is_empty(){
        println!("{}","arr is emtpy");
    } 

    arr.reverse();
    let b=arr.as_slice();
    for i in b.into_iter(){
        println!("{}",i.to_owned())
    }
}

#[test]
fn test_new_arr(){
    new_arr()
}