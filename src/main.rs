#[macro_use]
extern crate hello_world_derive;

use std::str::Chars;
use std::string::String as good;

// use json::JsonValue::String;

use ccrate::crate_learn::front_of_house as learn;
use match_ln::match_learn as mlearn;

mod ccrate;

mod hashmap_ln;
mod match_ln;
mod result_ln;

mod trait_ln;

mod string_ln;

mod slice_ln;

mod struct_ln;

mod flow_control_ln;

use flow_control_ln::for_range_learn;
use flow_control_ln::if_let_learn;
use flow_control_ln::loop_learn;

mod library_ln;

mod box_ln;
// use std::fmt::Debug;
// use std::string::String;

mod map_ln;

mod lifetime_ln;

mod reference_ln;

mod enum_ln;

mod thread_ln;

trait HelloWorld {
    fn hello_world();
}

#[derive(HelloWorld)]
struct FrenchToast;

#[derive(HelloWorld)]
#[HelloWorldName = "the best Pancakes"]
struct Waffles;

// mod macros_ln;
fn main() {
    learn::hosting::add_to_waitlist();
    hashmap_ln::map_learn::new();

    result_ln::result_learn::function_ok();
    let a = mlearn::value_in_cents(mlearn::Coin::Good);
    println!("{}", a);

    trait_ln::trait_learn::start_info();

    slice_ln::slice_learn::new_slice();

    string_ln::string_learn::new();
    struct_ln::user::fsss::bbbbb();
    struct_ln::user::use_user();
    // macros_ln::macros_learn::span();

    FrenchToast::hello_world();
    Waffles::hello_world();

    let ssss: good = "11111111".chars().rev().collect();
    println!("{}", ssss);
    // use std::collections::VecDeque;
    //
    // let a = [1, 2, 3];
    //
    // let doubled: VecDeque<i32> = a.iter().map(|&x| x * 2).collect();
    // println!(doubled)

    flow_control_ln::if_else_learn::test_if_else();

    loop_learn::test_loop();
    loop_learn::test_net_loop();
    loop_learn::test_return_loop();

    if_let_learn::let_learn();
    for_range_learn::test_for_range();
    for_range_learn::test_for_vec();
    for_range_learn::test_for_vec_mut();
    for_range_learn::test_for_tuple_destructuring();
    for_range_learn::test_for_guard();
    for_range_learn::test_for_guard_single();
    for_range_learn::test_for_guard_binding();
    for_range_learn::test_for_guard_binding_single();

    library_ln::start::start();

    box_ln::box_learn::box_test();

    map_ln::hashmap_test_entry();

    lifetime_ln::start();
    reference_ln::start_reference_learn();

    enum_ln::start();

    thread_ln::start();
}
