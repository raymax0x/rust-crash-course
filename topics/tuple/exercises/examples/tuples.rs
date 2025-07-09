#![allow(unused)]

fn return_empty_tuple() -> () {}

fn return_many() -> (u32, bool) {
    (12, true)
}

fn main() {
    let t: (bool, char, u32) = (true, 'a', 21);

    println!("{}, {}, {}", t.0, t.1, t.2);

    // empty tuple => unit type
    let t = ();

    // nested tuple
    let nested = (('a', -1i32, 1u32), (true, 1.23), ());

    println!("{}", (nested.1).1);

    // destructure tuple in rust
    let t: (bool, char, u32) = (true, 'a', 21);
    let (x, y, z) = t;

    println!("x {x}, y is {y}, z is {z}");

    let (a, b) = return_many();
}
