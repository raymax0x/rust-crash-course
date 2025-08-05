#![allow(unused)]

// Iterators adaptors
// - map, filter, collect

use std::collections::HashMap;

fn main() {
    // let vals: Vec<u32> = vec![1, 2, 3, 4, 5];

    // &u32
    // map - f(x: &u32) -> u32
    // with map we will able to take each element also can transform them to something else

    // val.iter().map(|x: &u32| {
    //     println!("x = {x}");
    //     println!("x = {x}");
    //     println!("x = {x}");
    //     println!("x = {x}");
    //     x + 1
    // });

    // collect
    // let v2: Vec<u32> = vals.iter().map(|x: &u32| x + 1).collect();
    // println!("v2 {:?}", v2);

    let vals: Vec<(&str, u32)> = vec![("a", 1), ("b", 2), ("c", 3)];

    let v: Vec<(String, u32)> = vals.iter().map(|v| (v.0.to_string(), v.1 + 1)).collect();
    println!("v vector {:?}", v);

    let v: HashMap<String, u32> = vals.iter().map(|v| (v.0.to_string(), v.1 + 1)).collect();
    println!("v hashmap {:?}", v);

    // Iterator adapter are function that we can call on our iterator.

    // Chaining filter and map
    let vals: Vec<u32> = vec![1, 2, 3, 4, 5];

    let v: Vec<u32> = vals
        .iter()
        .filter(|x: &&u32| **x <= 3)
        .map(|x: &u32| x + 1)
        .collect();

    println!("filter -> map {:?}", v);

    // lets use iter_into for same example

    let vals: Vec<u32> = vec![1, 2, 3, 4, 5];

    // here we get the value back whenever we use into_iter
    let v: Vec<u32> = vals
        .into_iter()
        .filter(|x: &u32| *x <= 3) // filter will return a reference to the thing being iterated.
        .map(|x: u32| x + 1) // for each iterator we are getting an actual value back.
        .collect();

    println!("into_iter -> filter -> map {:?}", v);
}
