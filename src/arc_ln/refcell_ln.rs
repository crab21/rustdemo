use std::cell::RefCell;

fn refcell_ln_variable(){
    let x = 5;
    let z = RefCell::new(x);
    let y =RefCell::borrow(&z);
    println!("{}", y);
}
#[test]
fn test_refcell_ln_variable(){
    refcell_ln_variable()
}
