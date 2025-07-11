#![allow(unused)]

// fn main() {
// panic!("something went wrong");

// let v = vec![1, 2, 3];
// // v[999]; // this will panic

// // so to handle such errors

// let x: Option<&i32> = v.get(99);

// match x {
//     Some(val) => println!("x:  {:?}", val),
//     None => println!("x: None"),
// }

// // Result<T,E> => Ok(T) | Err(E)

// let x = 1;
// let y = 0;

// // This will panic, when y = 0 (division by 0)
// // let q = x / y;

// #[derive(Debug)]
// enum MathError {
//     DivByZero,
//     Other,
// }

// let q: Result<i32, MathError> = if (y != 0) {
//     Ok(x / y)
// } else {
//     Err(MathError::DivByZero)
// };

// match q {
//     Ok(val) => println!("x/y = {:?}", val),
//     Err(err) => println!("x/y error {:?}", err),
// }
// }

// Summary

// 1. panic! -> crashes the program when something goes wrong
// 2. Option<T> = Some(T) //  option enum is used to express the prescence or the absence of a value, This works well when you want to access vector or array elements.

// 3. Result // Result enum , when something is successful we return okay with the value inside, when something fails we return error with the error type wrapped inside.

#[derive(Debug)]
pub enum MathError {
    DivByZero,
}

pub fn div(x: u32, y: u32) -> Result<u32, MathError> {
    let res: Result<u32, MathError> = if (y != 0) {
        Ok(x / y)
    } else {
        Err(MathError::DivByZero)
    };

    res
}

// pub fn get(v: &[u32], i: usize, default_val: u32) -> u32 {
//     let value: Option<&u32> = v.get(i);

//     match value {
//         Some(val) => *val,
//         None => default_val,
//     }
// }

// fn main() {
//     // let result = div(12, 0);
//     // println!("result {:?}", result);

//     let v = vec![1, 2, 4, 52, 3, 54, 2345, 2, 3, 23];
//     let res: u32 = get(&v, 2, 1000);
//     println!("res {:?}", res);
// }

pub fn get(v: &[u32], i: usize, default_val: u32) -> u32 {
    let value: Option<u32> = v.get(i).copied(); // Also optionally .copied() to avoid reference
    match value {
        Some(val) => val,
        None => default_val,
    }
}

fn main() {
    let v = vec![1, 2, 4, 52, 3, 54, 2345, 2, 3, 23];
    let res: u32 = get(&v, 999, 1000);
    println!("res {}", res);
}
