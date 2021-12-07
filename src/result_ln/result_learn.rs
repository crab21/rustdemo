use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("/root/learn/demo-lib/src/result_ln/hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(ass) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_abbreviation() -> Result<String, io::Error> {
    let mut f = File::open("/root/learn/demo-lib/src/result_ln/hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
pub fn function_ok() {
    let sr: Result<String, io::Error> = read_username_from_file_abbreviation();
    let sp = sr.ok();
    // println!("{:?}", sp);
    print_type_of(&sp)
    // match sr {
    //     Ok(s) => println!("{}", s),
    //     Err(e) => println!("{}", e),
    // }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
