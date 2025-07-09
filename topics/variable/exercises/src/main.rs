#![allow(unused)]
// Exercise: Fix the code to make it compile and pass the assertions
  
// Constants
const MAX: i32 = 100;    

fn main() {
    // Exercise 1: Make this variable mutable
    let mut count = 1;
    count += 1;

    println!("count: {count}");

    //  shadowing
   let x: i32 = 1;
   let x: i32 = 2;
   let x: bool = true;

   // Type placeholder
   let x: _ = 1234;

 

    
}
