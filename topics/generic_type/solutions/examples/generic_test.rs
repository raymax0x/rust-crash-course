#![allow(unused)]

// Option, Result, Vec

// enum Option<T> {
//     Some(T),
//     None,
// }

/*
enum Option<u32> {
    Some(u32),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
 */

// How to define your own generic type ?

// struct Point {
//     x: i32,
//     y: i32,
// }
struct Point<T> {
    x: T,
    y: T,
}

// fn swap(a: u32, b: u32) -> (u32, u32) {
//     (b, a)
// }

// for generic data type
fn swap<A>(a: A, b: A) -> (A, A) {
    (b, a)
}

fn swap_diff_para<A, B>(a: A, b: B) -> (B, A) {
    (b, a)
}

fn main() {
    // let v: Vec<T> = vec![1u32, 2, 3];
    let v: Vec<u32> = vec![1u32, 2, 3];

    // So basically creating a generic type allows us to add any sort of type we want while initializing its value, it could be i32, f32... anything.
    let p: Point<i32> = Point { x: 0, y: 0 };
    let p1: Point<f32> = Point { x: 12.2, y: 12.3 };

    let mut a = 1;
    let mut b = 2;

    (a, b) = swap(a, b);

    println!("swapped valu a {a}");
    println!("swapped valu b {b}");

    let mut a = "one";
    let mut b = "two";

    (a, b) = swap(a, b);

    println!("swapped valu a {a}");
    println!("swapped valu b {b}");

    // how to use this ?

    let a1: &str = "one";
    let b1: i32 = 2;

    let (a1, b1) = swap_diff_para(a1, b1);
    println!("swapped valu a1 {a1}");
    println!("swapped valu b1 {b1}");
}
