#![allow(unused)]

// Ownership rules
// 1. Each value has an owner
// 2. There can only be one owner at a time
// 3. when the owner goes out of scope, the value will be dropped.

// 4. However these rules can be ignored, FOR data type that implment the copy trait, like integer, static array.

fn take(s: String) {
    println!("take {s}");
    // s is dropped
}

fn copy(v: i32) {
    println!("copy {v}");
    // v is dropped here.
}

fn printArr(test_arr: [i32; 10]) {
    println!("test_arr print arr : {:?}", test_arr);
}

fn main() {
    // 1. Each value has an owner

    // the owner or this string "rust" is 's'
    let s = String::from("rust");

    // if you had a variable called i, which is equal to 1, then the owner of this is value is variable i.
    let i = 1;

    // Each data has its owner.

    // 2. There can only be one owner at a time

    // the owner of this string is s.
    let s = String::from("dog");

    // now if you reassingn this variable to s to another variable thats called s1, then the owner of the string "dog" will be transferred over to the variable s1.

    let s1 = s;

    // if you do this again, the owner of this string dog will again be transferred from s1 over to s2. since there can only one owner at a time

    let s2 = s1;
    println!("s2 : {s2}");

    // this wont work, the code wont even compile
    // println!("s : {s}")

    // this is because the owner of the string dog is now owned by the varaible s2. so the variable s is invalid.

    // 3. when the owner goes out of scope, the value will be dropped.
    // value being dropped, means that you can no longer reference that value.
    let s = String::from("cat");
    {
        // when we use the variable inside a new scope, the variable moves into that scope. So once the scope is done executing, this variable will b dropped.
        s;
        // varaible "s" is dropped here
    } // scope ends here.

    // the code wont compile here, since the value of s is dropped.
    // println!("s : {s}");

    let s = String::from("cat");
    {
        // owner of "cat" is s1
        let s1 = s;
        // varaible "s1" is dropped here
    } // scope ends here.
      // the code wont compile here, since the value of s is dropped.
      // println!("s : {s}");

    // the same logic goes for the function

    // once the function is executed the input will be dropped.
    let s = String::from("cat");
    take(s);
    // the code wont compile here, since the value of s is dropped.
    // println!("s : {s}");

    // Ownership does not move for types that implement the copy trait.
    // instead of the ownership transferring over, it copies the value so we dont have to worry about ownership.

    // owner of value 1 is i.

    // Here each time we create a variable and assign a value to it, the ownership is not transffered but each value is being copied to each variable. The integer type implements a copy trait. thats why.
    let i = 1;
    let i1 = i;
    let i2 = i1;

    // in the case for strings that does not implement the copy trade, when we pass the string into the function take, the function take took ownership of the string. And afterward we could not print it inside the main functions.

    // however, for data types that implements the copy trade, we can pass the data into the function without worrying about ownership and then use it inside the main function again.

    // here the value for i will be copied as input. So the ownership of i will stay inside the main functions.
    copy(i);
    println!("i : {i}");

    let arr: [i32; 10] = [0i32; 10];

    printArr(arr);
    println!("arr : {:?}", arr);
}
