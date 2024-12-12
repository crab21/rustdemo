

#[test]
fn closure_ln_test() {
    fn plus(x: i32) -> impl Fn(i32) -> i32 {
       move |a| a+x
    }
    fn map<T,U>(value: T,func: impl FnOnce(T) -> U) -> U {
        func(value)
    }

    fn assert_fn_once<F: FnOnce(i32) -> i32>(_: F) {}
    fn assert_fn_mut<F: FnMut(i32) -> i32>(_: F) {}
    let x = 5;

    let plus_x = plus(x);

    assert_fn_once(&plus_x); 
    assert_fn_mut(&plus_x);

    assert_eq!(map(2, plus_x),7)
}