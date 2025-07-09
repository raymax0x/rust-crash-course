#![allow(unused)]


fn add_with_return(x: u32, y: u32) -> (u32) {
    return(x+y);
}

fn add(x: u32, y: u32) -> (u32) {
    x+y
}

fn main() {
    add(1, 2);
}