use std::sync::Arc;
#[test]
fn test_arc_ln_cloning(){
    arc_ln_cloning()
}
// arc_ln_cloning
fn arc_ln_cloning(){
   let foo =  Arc::new(vec![1.0,2.0,3.0]);

   let a = foo.clone();
   let b = Arc::new(foo);
   println!("{:?} {:?}",a,b)
}
#[test]
fn test_deref(){
    arc_ln_deref()
}
fn arc_ln_deref(){
    let arc = Arc::new(());
    let wk = Arc::downgrade(&arc);
    println!("{:?} {:?}",arc,wk)
}
