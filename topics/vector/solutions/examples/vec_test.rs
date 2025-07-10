#![allow(unused)]

// Vector
fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("v: {:?}", v);

    let v: Vec<i8> = vec![1, 2, 3];
    let v = vec![1i8, 2, 3];

    // initialize a vector of 100 length having 0 at each index.
    let x: Vec<i8> = vec![10i8; 20];
    // println!("x: {:?}", x);

    // Get
    println!("v: {}", v[0]);

    // Option<&i8>
    // Index valid => some(&val)
    // Index invalid => None
    println!("v[1] : {:?}", v.get(1)); // Some(2)
    println!("v[100] : {:?}", v.get(100)); // None

    // Update
    let mut v: Vec<i8> = vec![1, 2, 3];
    v[0] = 99;

    // pop - remove last element
    let mut v: Vec<i8> = vec![1, 2, 3];
    let x: Option<i8> = v.pop();
    println!("pop: {:?}", x); // Some(3)

    let x: Option<i8> = v.pop();
    println!("pop: {:?}", x); // Some(2)

    let x: Option<i8> = v.pop();
    println!("pop: {:?}", x); // Some(1)

    let x: Option<i8> = v.pop();
    println!("pop: {:?}", x); // None

    // Slice
    let v = vec![1, 2, 3, 4, 5];
    let s = &v[1..4];
    println!("slice s: {:?}", s);
}
