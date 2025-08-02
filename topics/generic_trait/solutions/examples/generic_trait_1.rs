#![allow(unused)]

// we are creating a trait with boudation which is -> that we assume the datatype of whoever this trait is applied to is going to have store u32 integer
// trait List_bounded {
//     fn count(&self) -> usize;
//     fn first(&self) -> &u32;
// }

// to make it generic type
trait List<T> {
    fn count(&self) -> usize;
    fn first(&self) -> &T;
}

// Lets implement this list for a tuple.

impl List<u32> for (u32, bool, char) {
    fn count(&self) -> usize {
        3
    }

    fn first(&self) -> &u32 {
        &self.0
    }
}

// let implement our trait for vec
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
    println!("t.first {:?}", t.first());

    let v: Vec<u32> = vec![10, 2, 3, 4, 5];
    println!("v.count {}", v.count());
    println!("v.first {:?}", v.first());
}
