#![allow(unused)]

fn main() {
    // loop keyword
    let mut i = 0;
    loop {
        println!("loop {i}");
        i += 1;

        if i > 5 {
            break;
        }
    }

    // While loop
    let mut i = 0;
    while i <= 5 {
        println!("while loop {i}");
        i += 1;
    }

    // for loop
    for i in 0..6 {
        println!("for loop {i}");
    }

    // For array loop.
    let arr = [1, 2, 3, 4, 5];
    let n: usize = arr.len();
    for i in 0..n {
        println!("arr length {}", arr[i]);
    }

    //
    for m in arr {
        // m is actual element here.
        println!("Direct array loop {}", m);
    }

    // for loop in vector
    let v = vec![10, 20, 30, 40, 50];

    for j in v {
        // m is actual element here.
        println!("loop in vector {}", j);
    }

    // this wont run the second time because ownership concept
    // for j in v {
    //     // m is actual element here.
    //     println!("loop in vector {}", j);
    // }

    // if you want to run it twice, you have to use iter method on both loops
    // for n in v.iter() {
    //     println!("loop in 2nd time vector {}", n);
    // }
    // for n in v.iter() {
    //     println!("loop in 2nd time vector {}", n);
    // }

    // Return value from loop
    let mut i = 0;
    let z: &str = loop {
        println!("loop {i}");
        i += 1;

        if i > 5 {
            break "loop ends here";
        }
    };

    println!("loop return z {}", z);
}
