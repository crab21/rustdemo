use std::string::String as good;

// use json::JsonValue::String;

use ccrate::crate_learn::front_of_house as learn;
mod ccrate;

mod derive_ln;

mod hashmap_ln;
mod match_ln;
mod result_ln;
mod trait_ln;

mod string_str_ln;

mod slice_ln;

mod struct_ln;

mod flow_control_ln;

use flow_control_ln::for_range_learn;
use flow_control_ln::if_let_learn;
use flow_control_ln::loop_learn;
use serde_json::map;
mod shell_ln;

mod box_ln;
mod generic_ln;
mod library_ln;
// use std::fmt::Debug;
// use std::string::String;

mod map_ln;

mod lifetime_ln;

mod reference_ln;

mod enum_ln;

mod thread_ln;

mod fn_mut_ln;

mod option_ln;

mod iterator_ln;

mod tuple_ln;

mod mut_ln;

mod impl_ln;

mod anonymous_ln;
mod arc_ln;
mod io_ln;
mod std_ln;

mod tokio_ln;

mod cell_ln;
mod channel_ln;
mod librarys;

trait HelloWorld {
    fn hello_world();
}

// mod macros_ln;
fn main() {
    tuple_ln::sort_tuple_ln(&mut vec![(1, 2), (3, 4), (5, 6)]);
    match_ln::match_init_ln();

    learn::hosting::add_to_waitlist();
    hashmap_ln::map_learn::new();

    result_ln::result_learn::function_ok();

    trait_ln::trait_learn::start_info();

    slice_ln::start();

    string_str_ln::string_learn::new();
    struct_ln::user::fsss::bbbbb();
    struct_ln::user::use_user();
    // macros_ln::macros_learn::span();

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

    library_ln::start::sstart();

    box_ln::box_learn::box_test();

    map_ln::hashmap_test_entry();

    lifetime_ln::start();
    reference_ln::start_reference_learn();

    enum_ln::start();

    thread_ln::start();

    mut_ln::start_mut()
}
