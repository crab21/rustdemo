use std::fs::File;
use std::io::{self, BufReader};
use std::io::{prelude::*, SeekFrom};
use std::string;

static f: &str = "/Users/gogoowang/learn/rustdemo/src/io_ln/mod.rs";

// read lines
fn read_buf_lines() -> io::Result<()> {
    let open_file = File::open(f)?;
    // let mut buf = [0; 10];
    // let n = open_file.read(&mut buf)?;
    let result = BufReader::new(open_file);
    let a = result.lines();
    for line in a {
        println!("{:?}", line.unwrap());
    }
    Ok(())
}

fn read_buf_bytes() -> io::Result<()> {
    let mut open_file = File::open(f)?;
    let mut buf = [0; 1];
    let n = open_file.read(&mut buf)?;
    let result = &buf[..n];
    let u8s = String::from_utf8(result.to_vec()).unwrap_or_else(|e| {
        let es = e.to_string();
        panic!("{:?}", es);
    });

    println!("{:?}", u8s);

    Ok(())
}

fn read_buf_bytes_loop() -> io::Result<()> {
    let mut open_file = File::open(f)?;
    let mut buf = [0; 4];
    loop {
        let n = open_file.read(&mut buf)?;
        let c = open_file.stream_position()?;
        println!("{:?} {:?}", String::from_utf8(buf[..n].to_owned()), c);
        if n != 4 {
            break;
        }
    }

    Ok(())
}

#[test]
fn test_read_buf() {
    let result = read_buf_lines();
    println!("{:?}", result);

    let result_bytes = read_buf_bytes_loop();
    println!("{:?}=============", result_bytes);
}
