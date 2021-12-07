pub fn test_loop() {
    let mut i = 0;
    loop {
        i += 1;
        if i == 10 {
            println!("{}", i);
            break;
        }
    }
}

pub fn test_net_loop() {
    let mut i = 0;
    'outer: loop {
        println!("outer good");
        'inner: loop {
            println!("inner good");
            i += 1;
            if i == 10 {
                break 'outer;
            }
        }
    }
}

pub fn test_return_loop() {
    let mut i = 0;
    i = loop {
        i += 1;
        if i == 10 {
            break i * 10;
        }
    };
    println!("{}", i);
}
