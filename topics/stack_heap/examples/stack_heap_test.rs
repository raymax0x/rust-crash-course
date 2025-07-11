#![allow(unused)]

fn main() {
    // the following examples are for stack.
    // this is going to stored in stack.
    // since the size of this is known at compile time.
    let x: i32 = 1;
    // another example is fixed size array
    // as you can see 32 bits of interger of 10 size which is 320 bits, so compiler can calculate the size
    // and will store them on stack
    let arr: [i32; 10] = [1; 10];

    // --------------

    // ---- Lets go for HEAP ----

    // The reason why the string is stored on the heap is becuase of the size of the string can change when the program executes, Hence the size of the string is not known at compile time
    let mut s: String = "hello".to_string();
    s += " world";

    // As you can see the size of vector changes on the runtime (as the code keeps executing)
    let mut v = vec![];
    v.push(0);
    v.push(0);
    v.push(0);

    // We could force any data to be stored on the heap.

    // this will put the data in heap, if this is not wrapped in boxed then i32 will be stored on the stack.
    let boxed = Box::new(1i32);
}
