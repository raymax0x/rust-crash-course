// Borrow - temporarily use a value without taking ownership
// - Creates a reference (either mutable or immutable)
// - Doesn't move ownership
// - Immutable reference - any number of read-only access to a value
// - Mutable reference - only one read and write access to a value at a time
// - Either immutable or mutable borrow, but not both simultaneously.
// - Reference must not outlive the value

#![allow(unused)]

fn take(s: String) {
    println!("take {s}");
}

// Borrow: temporarily use a value without ownership
// this example can be considered to use for all data types that does not implements the copy trade.

// So to use the borrow for the data variable we are concerned with, we need to use the reference.
// the reference can either be mutable or immutable.

// - Creates a reference (either mutable or immutable)

// When we create a refference, the borrowing rules applies and the data does not move ownership,

// - Immutable reference - any number of read-only access to a value
// - Mutable reference - only one read and write access to a value at a time. ( we have a reference to a variable that we can mutate. Therefore only one read and write acess to a value at a time can be created. )

fn borrow(s: &String) {
    println!("borrow {s}");
}

fn main() {
    // Take Ownership
    let s = String::from("rust");
    // take(s);
    borrow(&s);

    // s is dropped after take(s)
    // this will not compile
    println!("s: {s}");

    // Immutable borrow
    // let s = String::from("dog");

    // to create a immutable reference we have to do is create a referene to the variable by putting the & before the variable.

    // let s1 = &s;
    // let s2 = &s;

    // since s2 is variable refernce to "s", s3 will also be a variable reference to s.
    // let s3 = &s2;

    // So s1, s2, s3 all have read only access to s.

    // Mutable Borrow
    // let mut s = String::from("rust");
    // let s1 = &mut s;
    // let s2 = &mut s; // this is example for showcasing that 2 mutable referrene at a time is invalid and will make the compiler failure.

    // now s1 has read and write access to s.
    // s1.push_str("😁");

    // Only one mutable reference existing at a time.
    // let s2 = &mut s; // this code won't compile
    // s2.push_str("😁");
    // println!("s2 {s2}");

    // - #4 : Either immutable or mutable borrow, but not both simultaneously.
    // let mut s = String::from("rust");
    // s1, s2 and s3 have read only access to s
    // let s1 = &s; // immutable reference 1.  here we say this is immutable reference
    // let s2 = &s; // immutable reference 2.

    // let s3 = &mut s; // mutable reference. //error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable

    // println!("s1: {s1}");
    // s3.push_str("😁");

    // ---------------- 5 - Reference must not outlive the value
    // let s = String::from("rust");
    // let s1 = &s;
    // {
    //     let s2 = s;
    // }
    // s2 and s are no longer exist.
    // but still we have s1 which is referecen of s.
    // so this is it : Reference is outlive the value
    // println!("s1 : {s1}"); // this wont compile.
}

// another example of rule 5 : Reference is outlive the value

// this wont compile as well :->

// you are returning an reference of the String s, when the function is called the ownership of string s moves inside the function. when the function is done executing this string "s" will be dropped, however we are returning a reference to a string. therefore  returning a reference but the value has already been dropped.
// fn dangle(s: String) -> &String {
//     &s
// }

// Summary

// - We can borrow values by creating a reference.
// - When we create a reference, it does not take ownership, it simply borrows the value.
// - We can create either a immutable reference or a mutable reference, but not both at the same time.
// - we can create multiple immutable references simultaneously, however we can only have one mutable reference at a time.
