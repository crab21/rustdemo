
use futures::executor::block_on;

async fn hello_world() {
    println!("hello, world!");
}

#[test]
fn async_ln_test() {
    let future = hello_world();
    println!("{:?}", block_on(future));
}