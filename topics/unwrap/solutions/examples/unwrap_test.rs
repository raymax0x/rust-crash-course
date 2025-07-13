#![allow(unused)]

// fn main() {
//     let x: Option<i32> = Some(3);
//     let v: i32 = match x {
//         Some(val) => val,
//         None => panic!("no value"),
//     };

//     // Unwraps the inner value, Panics if None
//     let i = x.unwrap();
//     println!("{}", i);

//     let x: Result<i32, String> = Ok(3);
//     let v: i32 = match x {
//         Ok(val) => val,
//         Err(err) => panic!("err: {:?}", err),
//     };

//     let i = x.unwrap();
//     println!("result: {}", i);

//     // err

//     // let x: Option<i32> = None;
//     let x: Result<i32, String> = Err("error".to_string());
//     let i = x.unwrap();
//     println!("result err: {}", i);

//     // expect
//     let x: Result<i32, String> = Err("error in x val".to_string());
//     x.expect("something failed");
// }

// Long way ->
// pub fn parse_and_add(a: &str, b: &str) -> u32 {
//     let val_a: Result<u32, std::num::ParseIntError> = a.parse();
//     let val_b: Result<u32, std::num::ParseIntError> = b.parse();

//     // using unwrap
//     let unwrap_a = val_a.unwrap();
//     let unwrap_b = val_b.unwrap();

//     // using expect

//     let x = unwrap_a + unwrap_b;
//     x
// }

// Short way ->
// pub fn parse_and_add(a: &str, b: &str) -> u32 {
//     let val_a: u32 = a.parse().expect("Reason");
//     let val_b: u32 = b.parse().expect("Reason");
//     let x = val_a + val_b;
//     x
// }

// with unwrap ->
// pub fn parse_and_add(a: &str, b: &str) -> u32 {
//     let val_a: u32 = a.parse().unwrap();
//     let val_b: u32 = b.parse().unwrap();
//     let x = val_a + val_b;
//     x
// }

pub fn unwrap_and_add(x: Option<u32>, y: Option<u32>) -> u32 {
    x.unwrap() + y.unwrap()
    // x.expect("x err") + y.expect("y err")
}

fn main() {
    let val_x: Option<u32> = Some(2);
    let val_y: Option<u32> = Some(2);

    let res: u32 = unwrap_and_add(val_x, val_y);
    println!("res: {}", res);
}
