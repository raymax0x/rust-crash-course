fn parse(s: &str) -> Result<u32, String> {
    match s.parse() {
        Ok(val) => Ok(val),
        Err(_) => Err("Failed to parse string into u32".to_string()),
    }
}

pub fn sum(nums: &[&str]) -> Result<u32, String> {
    let mut total: u32 = 0;

    for i in nums {
        let parsed_val = parse(&i)?;
        total += parsed_val;
    }

    Ok(total)
}

fn main() {
    let arr = ["1", "2", "3"];
    let result = sum(&arr);
    println!("sum of arr : {:?}", result);
}
