// https://doc.rust-lang.org/book/ch08-02-strings.html

pub fn new(){
   append1();

   append2();

   append3();

   string_to();
}

 fn append1(){
    let mut s = String::new();
    let s1 = String::from("hello");
    let s2: &str = "hello";
    let s3 = s+s1.as_str();
    println!("{}", s3);
}

fn append2(){
    let mut s = String::new();
    s.push_str("good");
    println!("{}", s);
}
fn append3(){
    let mut s  = String::from("hello-----------");
    let mut s1= String::from("world");
    for c in s1.chars(){
        s.push(c);
    }

    println!("{}", s);
}


fn string_to(){
    let s = String::from("👉🏻");

    let result = s.encode_utf16().collect::<Vec<u16>>();
    println!("{:?}", result);

   let a =  String::from_utf16(&result).unwrap();
   println!("{}", a);
}