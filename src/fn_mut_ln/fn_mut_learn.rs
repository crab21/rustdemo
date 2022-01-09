#[test]
fn test_fn_mut() {
    fn fn_immut<F>(f: F)
    where
        F: FnOnce() -> String,
    {
        println!("----------------->calling Fn closure from fn, {}", f());
    }

    let a = "Fn".to_string();
    fn_immut(|| a.clone()); // 闭包返回一个字符串
}

#[test]
fn test_fn_mut_1() {
    fn fn_immut<F>(f: F)
    where
        F: FnOnce() -> String,
    {
        println!("calling Fn closure from fn, {}", f());
    }

    let a = "Fn".to_string();
    fn_immut(|| a.clone()); // 闭包返回一个字符串
}
