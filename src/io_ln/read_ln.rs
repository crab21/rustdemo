use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;
#[test]
fn io_read() {
    // 在生成输出之前，文件主机必须存在于当前路径中
    if let Ok(lines) = read_lines("./hosts") {
        // 使用迭代器，返回一个（可选）字符串
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }
}

// 输出包裹在 Result 中以允许匹配错误，
// 将迭代器返回给文件行的读取器（Reader）。
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_to_string_ln() {
    // let content = fs::read_to_string("/Users/gogoowang/learn/rustdemo/src/io_ln/mod.rs")
    // .expect("Should have been able to read the file");

    // println!("{}",content);
    let cc = fs::read_to_string("/Users/gogoowang/learn/rustdemo/src/io_ln/mod.ffrs");
    match cc {
        Ok(c) => println!("{}", c),
        Err(e) => println!("err: {}", e),
    };
}

#[test]
fn read_to_string_ln_test() {
    read_to_string_ln()
}
