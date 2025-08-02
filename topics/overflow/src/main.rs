#![allow(unused)]

fn main() {
    // let mut x = u32::MAX;
    // println!("x value is {x}");
    // x += 1;
    // println!("x value is {x}");

    // u32::checked_add - return none on Overflows
    let x = u32::checked_add(u32::MAX, 1);
    println!("checked_add: {:?}", x);

    let x = u32::checked_add(3, 1);
    println!("checked_add: {:?}", x);

    // u32::wrapping_add - explicitly allow overflows
    let x = u32::wrapping_add(u32::MAX, 1);
    println!("wrapping_add: {:?}", x);

    let x = u32::wrapping_add(3, 1);
    println!("wrapping_add: {:?}", x);
}
