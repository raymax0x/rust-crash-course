pub fn parse_and_add(a: &str, b: &str) -> u32 {
    let val_a: u32 = a.parse().expect("Failed to parse variable A");
    let val_b: u32 = b.parse().expect("Failed to parse variable B");
    let x = val_a + val_b;
    x
}

pub fn unwrap_and_add(x: Option<u32>, y: Option<u32>) -> u32 {
    x.unwrap() + y.unwrap()
}
