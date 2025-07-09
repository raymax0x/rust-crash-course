#![allow(unused)]

fn main() {
    // let msg: String = String::from("hello");
    let msg: String = "hello world".to_string();
    println!("{msg}");

    let length: usize = msg.len();
    println!("{length}");

    let msg: String = String::from("hello");
    let s: &str = &msg[0..5];
    println!("{s}");

    let s: &str = "Hello World";
    let x: String = s.to_string();

    // Rust automatically converts string into a &str
    let msg: String = String::from("Hello Rust print");
    print(&msg); // as you can see this works, since the rust automatically converts string into a slice of string

    let s: &str = "Hello World String Slice";
    print(s);

    // Append &str to String
    let mut msg: String = String::from("Hello Rust");
    msg += " World";
    println!("{msg}");

    // String interpolation - format!
    let lang = "Rust";
    let emoji = "😁";
    let s = format!("Hello {} {}", lang, emoji);
    println!("{s}");
}

fn print(s: &str) {
    println!("{s}");
}
