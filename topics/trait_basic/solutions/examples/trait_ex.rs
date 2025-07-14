pub trait Tester {
    fn test(&self, file_path: &str) -> String;
}

pub struct Foundry {
    pub version: String,
}

impl Tester for Foundry {
    fn test(&self, file_path: &str) -> String {
        format!("forge test {}", file_path)
    }
}

fn main() {
    let f = Foundry {
        version: "0.2".to_string(),
    };

    println!("foundry imple : {}", f.test("xyz.fo"));
}
