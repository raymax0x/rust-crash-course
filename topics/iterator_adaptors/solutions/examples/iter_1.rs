#![allow(unused)]

// Iterators
fn main() {
    let vals: Vec<u32> = vec![1, 2, 3, 4, 5];

    for v in vals() {
        println!("v {}", v);
    }

    //  when you use loop on vector, it inferally calls "vals.into_iter()" so if you want to make two loops call you have to use .iter() method.

    // Dif betweent : "into_iter" and "iter"

    // 1. Both produces an iterator.
    // Iterator is a trait that says that the data type that implements the iterator can be iterated.

    //1. into_iter - type T 
    for v: u32 in vals.into_iter() {
        println!("v {}", v);
    }

    //2.  iter - iterator &T - this is giving back the reference to the value being iterated.
    for v: &u32 in vals.iter() {
        println!("v {}", v);
    }

    //3. iter_mut - iterate &mut T - this gives mutable reference
    for v in vals.iter_mut() {
        println!("v {}", v);
    }

    // we cant loop vector twice using this syntax.
    for v in vals.iter() {
        println!("v {}", v);
    }
}
