#[test]
fn t_ln() {
    let number_list:Vec<i32> =Vec::new();
    if number_list.len()==0{
        return ;
    }
    let mut max = number_list.iter().min().unwrap();
    println!("{}", max);
    for (i,number) in number_list.iter().enumerate(){
        println!("{}",i);
        println!("{}",number);
    }

    for nu in number_list.iter(){
        if nu > max{
            max = nu;
        }
    }

    println!("{}", max);
}
