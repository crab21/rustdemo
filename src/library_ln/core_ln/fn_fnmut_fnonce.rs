pub fn hello() {
    println!("{}", "hello");
}

fn do_twice<F>(mut func: F)
where
    F: FnMut(),
{
    func();
    func();
}

pub fn dd() {
    let mut x: usize = 1;
    {
        let add_two_to_x = || x += 2;
        do_twice(add_two_to_x);
    }

    println!("x is: {}", x);
}
