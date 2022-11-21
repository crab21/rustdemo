use std::fmt;

#[test]
fn space_ln_test(){
    space_ln()
}

pub fn space_ln() {
    let spaces = "   ";
    let spaces = spaces.len();
    println!("{}", spaces)
}

#[test]
fn mute_ln_test(){
    mut_ln()
}
pub fn mut_ln(){
        let mut x: i32 = 100;
    
        // 可变借用
        let y: &mut i32 = &mut x;
    
        // y的类型是&mut i32，无法使用'+='，*y指向实际资源。使用'+='修改资源值。
        *y += 100;
    
        println!("{}", *y);    // 打印输出 200 。
        println!("{}", x);    // 打印输出 200 。
        // 或者，可以写成。 println!("{}", y);
}

