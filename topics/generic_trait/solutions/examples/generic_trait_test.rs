#![allow(unused)]

// This is how you define a generic trait
trait List<T> {
    fn count(&self) -> usize;
    fn first(&self) -> &T;
}

impl List<u32> for (u32, bool, char) {
    fn count(&self) -> usize {
        3
    }
    fn first(&self) -> &u32 {
        &self.0
    }
}

/*
    The generic Vec of type T, Implements the List<T> (trait) and this type T That we declare over here is a generic type.
*/

impl<T> List<T> for Vec<T> {
    fn count(&self) -> usize {
        self.len()
    }
    fn first(&self) -> &T {
        &self[0]
    }
}

fn main() {
    let t = (1u32, true, 'c');

    println!("t.count {}", t.count());
    println!("t.first {}", t.first());

    let v: Vec<u32> = vec![1, 2, 3, 4, 5];
    println!("t.count vector : {}", v.count());
    println!("t.first vector : {}", v.first());
}
