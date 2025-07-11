#![allow(unused)]

fn main() {
    let x: i32 = 2;
    match x {
        1 => println!("one"),
        2 => println!("second"),
        3 => println!("three"),
        _ => println!("others"),
    }

    match x {
        1 | 2 | 3 => println!("1 or 2 or 3"),
        _ => println!("others"),
    }

    let x = 10;
    match x {
        1..=10 => println!("from 1 to 10"),
        _ => println!("others"),
    }

    let x = 5;
    match x {
        i @ 1..=10 => println!("from 1 to 10 {i}"),
        _ => println!("others"),
    }

    let x: Option<i32> = Some(9);
    let x: Option<i32> = None;

    match x {
        Some(val) => println!("Option is {val}"),
        None => println!("None"),
    }

    let res: Result<i32, String> = Ok(123);
    let res: Result<i32, String> = Err("no number Error".to_string());

    match res {
        Ok(val) => println!("ok {val}"),
        Err(err) => println!("err : {err}"),
    }

    let x: Option<i32> = Some(9);
    let x: Option<i32> = None;
    let z: i32 = match x {
        Some(val) => val,
        None => 0,
    };

    println!("z is {z}");
}
