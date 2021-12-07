use std::collections::HashMap;

pub fn new() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("{:?}", map);
}
