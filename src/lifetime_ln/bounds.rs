use std::{fmt::Debug, borrow::Borrow}; // Trait to bound with.

#[derive(Debug,Clone, Copy)]
struct Ref<'a, T: 'a>(&'a T);
// `Ref` contains a reference to a generic type `T` that has
// an unknown lifetime `'a`. `T` is bounded such that any
// *references* in `T` must outlive `'a`. Additionally, the lifetime
// of `Ref` may not exceed `'a`.

impl<T> Ref<'_,T> {
    fn GetValue(&self)-> &T{
        let b = self.borrow();
        b.0
    }
}
// A generic function which prints using the `Debug` trait.
fn print<T>(t: T) where
    T: Debug {
    println!("`print`: t is {:?}", t);
}

// Here a reference to `T` is taken where `T` implements
// `Debug` and all *references* in `T` outlive `'a`. In
// addition, `'a` must outlive the function.
fn print_ref<'a, T>(t: &'a T) ->&T 
where
    T: Debug + 'a {
    println!("`print_ref`: t is {:?}", *t);
    t
}
#[test]
fn test_bounds() {
    let x = 7;
    let ref_x = Ref(&x);

    let a = print_ref(&ref_x);
    // print(ref_x);
    let b = a.GetValue();
    println!("{:?}",b)
}
//////////////////////////////////////
/// 
/// 
/// 
struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

// These functions are only valid for types which implement these
// traits. The fact that the traits are empty is irrelevant.
fn red<T: Red>(_: &T)   -> &'static str { "red" }
fn blue<T: Blue>(_: &T) -> &'static str { "blue" }

#[test]
fn test_blue_red_bound() {
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey   = Turkey;

    // `red()` won't work on a blue jay nor vice versa
    // because of the bounds.
    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
    //println!("A turkey is {}", red(&_turkey));
    // ^ TODO: Try uncommenting this line.
}
