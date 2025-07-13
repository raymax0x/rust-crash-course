// pub fn first(t: (u32, i32)) -> u32 {
//     t.0
// }

pub fn first<A>(t: (A, B)) -> A {
    t.0
}

// pub fn last(t: (u32, i32)) -> i32 {
//     t.1
// }

pub fn last<A, B>(t: (A, B)) -> B {
    t.1
}

// #[derive(Debug)]
// pub struct Rectangle {
//     pub top: u32,
//     pub left: u32,
//     pub width: u32,
//     pub height: u32,
// }

#[derive(Debug)]
pub struct Rectangle<A> {
    pub top: A,
    pub left: A,
    pub width: A,
    pub height: A,
}
