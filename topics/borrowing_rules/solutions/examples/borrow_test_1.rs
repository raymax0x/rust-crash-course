#![allow(unused)]

// 1st exercise can be solved using two apporoaches

// 1st approach -> changing the order of the print statement
// fn main() {
//     let mut s = String::from("Rust");
//     let s1 = &mut s;
//     println!("s1: {s1}");

//     let s2 = &mut s;
//     println!("s2: {s2}");
// }

// 2nd approach -> changing the mutable reference to immutable refference.
// fn main() {
//     let s = String::from("Rust");
//     let s1 = &s;
//     let s2 = &s;

//     println!("s1: {s1}");
//     println!("s2: {s2}");
// }

// fn main() {
//     let mut s = String::from("Rust");
//     let s1 = &mut s;
//     // let s2 = &mut s;

//     s1.push_str("!!!");
//     println!("s: {s}");
// }

fn print_len(s: &String) {
    println!("len: {}", s.len());
}

fn main() {
    let s = String::from("Rust");
    print_len(&s);
    println!("s: {s}");
}
