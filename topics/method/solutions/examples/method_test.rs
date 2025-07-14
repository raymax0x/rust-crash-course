#![allow(unused)]

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// A method is simply a function that is attached to a datatype

// to declare a method
impl Point {
    // Static Method - associated function
    // Self with the capital "S" refers to type, in this case this will point

    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    // Method : a function that operates on an instance of a point/type
    fn move_to(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}

fn main() {
    // let p = Point { x: 0.0, y: 0.0 };
    // p.move_to()

    // How to use Method ->
    let mut p = Point { x: 0.0, y: 0.0 };
    p.move_to(1.0, 2.0);
    println!("point p: {:?}", p);

    // How to use Static Method ->
    let mut q = Point::new(3.0, 4.0);
    println!("point q: {:?}", q);
}
