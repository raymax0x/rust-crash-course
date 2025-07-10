#![allow(unused)]

// Struct
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct Point3D(i32, i32, i32);

struct Empty;

#[derive(Debug)]
struct Circle {
    radius: u32,
    center: Point,
}

fn main() {
    let p1 = Point { x: 1, y: 1 };
    println!("{:?}", p1);
    println!("x : {}, y : {}", p1.x, p1.y);

    let p = Point3D(-1, 0, -1);
    println!("Point3d: {},{},{} ", p.0, p.1, p.2);

    let empty = Empty;

    // nested struct
    let circle = Circle {
        radius: 12,
        center: p1,
    };

    println!("circle : {:?}", circle);

    // printing in preety manner
    println!("circle : {:#?}", circle);

    // shortcut
    let x: i32 = 1;
    let y: i32 = 1;

    let p = Point { x: x, y: y };
    let p = Point { x, y };

    // Copy fields
    let p2 = Point { x: 1, y: 2 };
    let p3 = Point { x: 2, ..p2 };

    println!("p2 copy: {:?}", p3);

    // Update
    let mut p1 = Point { x: 1, y: 1 };
    p1.x += 1;
    p1.y = 99;
    println!("update point: {:?}", p1);
}
