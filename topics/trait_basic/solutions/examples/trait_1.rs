#![allow(unused)]

// Trait

struct Solidity {
    version: String,
}

struct Vyper {
    version: String,
}

trait Compiler {
    fn compile(&self, file_path: &str) -> String;

    // this is default function for any struct which implements this trait and calls it.
    fn help(&self) -> String {
        "Good Luck!".to_string()
    }
}

// implementing a trait ->
impl Compiler for Solidity {
    fn compile(&self, file_path: &str) -> String {
        format!("solc {}", file_path)
    }
}
impl Compiler for Vyper {
    fn compile(&self, file_path: &str) -> String {
        format!("vyper {}", file_path)
    }
}

// So now we have the struct (solidity and vyper) that implements the compiler trait

// In rust all data sizes must be known at compile time
fn compile(lang: &impl Compiler, file_path: &str) -> String {
    lang.compile(file_path)
}

/*

There are two parts to trait ->
1. Defining a trait
2. Implementing a trait
*/

fn main() {
    let sol = Solidity {
        version: "0.8".to_string(),
    };
    let vy = Vyper {
        version: "0.4".to_string(),
    };

    // First way to call the function compile on solidity and vyper struct instances.
    println!("sol compile: {}", sol.compile("hello.sol"));
    println!("vyper compile: {}", vy.compile("hello.vy"));

    // Other way
    println!("sol compile: {}", compile(&sol, "hello.sol"));
    println!("vyper compile: {}", compile(&vy, "hello.vy"));

    println!("sol compile: {}", sol.help());
    println!("vyper compile: {}", vy.help());
}

/*

    These are two ways to call the function to compile on a type that implements the compiler trait.

    We can directly call it
    or we can pass it into a function where the function takes in a trait as input.


*/
