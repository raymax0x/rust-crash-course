#![allow(unused)]

// SPLIT MODULES INTO FILES
use modules_practice::{foo, my};

fn main() {
    println!("Hello, world!");

    my::print();
    my::a::print();
    let s = my::a::build(9);
    let f = my::a::print_foo();
}
