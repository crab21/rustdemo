
use std::cell::Cell;

struct Point {
    x: u8,
    y: u8,
    cached_sum: Cell<Option<u8>>
}

impl Point{
    fn sum(&self)->u8{
        match  self.cached_sum.get(){
            Some(k)=>{
                println!("Got from cache: {}", k);
                k
            }
            None=>{
                let new_sum = self.x+self.y;
                self.cached_sum.set(Some(new_sum));
                println!("Set cache: {}", new_sum);
                new_sum
            }
        }
    }
}

#[test]
fn t_cell(){
    let p = Point{x:8,y:9,cached_sum: Cell::new(None)};
    println!("Summed result: {}", p.sum());
    println!("Summed result: {}", p.sum());
    let aa = get().unwrap();
    println!("{:?}",aa)
}


use std::net::UdpSocket;


pub fn get() -> Option<String> {
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(_) => return None,
    };

    match socket.connect("8.8.8.8:53") {
        Ok(()) => (),
        Err(_) => return None,
    };

    match socket.local_addr() {
        Ok(addr) => return Some(addr.ip().to_string()),
        Err(_) => return None,
    };
}