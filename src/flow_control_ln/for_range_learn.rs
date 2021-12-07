pub fn test_for_range() {
    let mut x = 0;
    for i in 0..10 {
        x += i;
    }
    assert_eq!(x, 45);
    println!("{}", x);
}
