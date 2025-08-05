#![allow(unused)]

use std::collections::HashMap;
fn main() {
    let v: Vec<(String, u32)> = vec![
        ("a".to_string(), 1),
        ("b".to_string(), 2),
        ("c".to_string(), 3),
        ("d".to_string(), 4),
    ];
    println!("v {:?}", v);

    // convert to hashmap
    let temp: HashMap<String, u32> = v.into_iter().collect();

    println!("temp hashmap {:?}", temp);
}
