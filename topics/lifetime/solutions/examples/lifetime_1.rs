#![allow(unused)]

// Lifetime - every referecne has a lifetime
// Lifetime tells the rust compiler how long the reference is valid for.

fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// this was the example of how lifetime err could occur.
// fn main() {
//     let x = "Hello".to_string();
//     let z = {
//         let y = "Rust".to_string();
//         longest_str(&x, &y)
//         // y is dropped.
//     };
//     println!("longest {:?}", z);
// }

fn print_refs<'a, 'b>(x: &'a str, y: &'b str) {
    println!("{} {}", x, y);
}

// Declaring a lifetime for a struct.
#[derive(Debug)]
struct Book<'a> {
    title: &'a str,
}

// Declaring a lifetime for a method.
impl<'a> Book<'a> {
    fn edit(&mut self, new_title: &'a str) {
        self.title = new_title;
    }
}

/**
Every ref has a lifetime and its used to ensure memory safety and to make sure that references dont become invalid when the program runs.
*/
fn main() {
    let x = "Hello".to_string();
    let y = "Rust".to_string();
    let z = longest_str(&x, &y);
    println!("longest {:?}", z);

    // Static lifetime : meaning the reference is static, the variable will be valid until the end of the program.
    let s: &'static str = "Hello";

    // Placeholder lifetime - let rust infer the lifetime
    let s: &'_ str = "Rust";
}
