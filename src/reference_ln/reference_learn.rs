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
