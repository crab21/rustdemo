use std::collections::HashMap;

pub fn map_insert() {
    let mut mp: HashMap<i32, i32> = HashMap::new();
    mp.insert(100, 2);

    for i in 0..mp.len() {
        println!("len: {}", i);
    }

    for v in mp.keys() {
        println!("keys: {}", v);
    }

    for v in mp.values() {
        println!("values: {}", v);
    }

    for (k, v) in mp.iter() {
        println!("k: {} \t v:{}", k, v);
    }

    mp.entry(100).or_insert(3);
}

pub fn map_iter() {
    let mut mp: HashMap<i32, i32> = HashMap::new();
    mp.insert(100, 2);

    for (k, v) in mp.iter() {
        println!("k: {} \t v:{}", k, v);
    }
}
