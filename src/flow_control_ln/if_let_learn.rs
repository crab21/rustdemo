enum FOOs {
    BAR,
    BAZ,
    QUX(u32),
}

pub fn let_learn() {
    let a = FOOs::BAR;
    if let FOOs::BAR = a {
        println!("a is BAR");
    }

    let c = FOOs::QUX(100);
    if let FOOs::QUX(value @ 100) = c {
        println!("c is QUX and x is {}", value);
    }

    if let FOOs::QUX(value) = c {
        println!("c is {}", value);
    }
}
