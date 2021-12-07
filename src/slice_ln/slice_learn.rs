


pub fn new_slice(){
    let s: String = String::from("hello word");
    let hello  = &s[0..3];

    println!("{:}", hello);
}