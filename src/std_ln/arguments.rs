

fn args_ln(){
    let args :Vec<String> = std::env::args().collect();
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);
}
#[test]
fn args_ln_test(){
    args_ln()
}