fn main() {
    let i: i32 = -21;
    let u: u32 = i as u32;

    println!("{i} as u32 is {u}");

    let i_max = i32::MAX;
    let u_min = u32::MIN;
    println!("i max is {i_max}");
    println!("u min is {u_min}");
}
