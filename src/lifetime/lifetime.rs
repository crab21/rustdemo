#[test]
fn test_overriding_lifetime() {
    let x = 1;
    let c = 'c';
    let a: String = String::from("value");

    match c {
        x => println!("x: {} c: {}", x, c),
    }

    println!("x: {}", x);
}
