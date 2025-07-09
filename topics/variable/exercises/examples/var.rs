

fn main () {
    let x = 3;

    // inline
    println!("x: {}",x);

    // 
    println!("x: {x}");

    // Positional
    println!("{0} + {0} = {1}", x, x + x);

    // Named
    println!("{} days", 21);

    // Debug
    println!("DEBUG: x {:?}", x);
    println!("DEBUG: X {:#?}", x);
}