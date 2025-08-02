#![allow(unused)]
use std::cmp::PartialOrd;

// Trait bound

fn max<T: PartialOrd>(x: T, y: T) -> T {
    if x <= y {
        x
    } else {
        y
    }
}

trait A {}
trait B {}
trait C {}

impl A for u32 {}
impl B for u32 {}
impl B for f32 {}
impl C for i32 {}

fn a<T: A>(x: T) {}

fn ab<T: A + B>(x: T) {}

// complicated syntax
// fn w<T: A + B, U: B + C>(x: T, y: U) {}

// easy syntax
fn w<T, U>(x: T, y: U)
where
    T: A + B,
    U: B + C,
{
}

fn main() {
    let u: u32 = 1;
    let i: i32 = -1;
    let f: f32 = 0.0;
    a(u);
    // a(i);

    ab(u);
    ab(f);
}
