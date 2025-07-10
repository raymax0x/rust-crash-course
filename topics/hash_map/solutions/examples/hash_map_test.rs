#![allow(unused)]

use std::collections::HashMap;

// HashMap
fn main() {
    let mut scores: HashMap<String, u32> = HashMap::new();
    scores.insert("red".to_string(), 100);
    scores.insert("blue".to_string(), 200);

    println!("{:#?}", scores);

    // Get
    let score: Option<&u32> = scores.get("red");
    println!("Red Score: {:?}", score);

    let score: Option<&u32> = scores.get("green");
    println!("Green Score: {:?}", score);

    // Update
    let score: &mut u32 = scores.entry("black".to_string()).or_insert(0);
    *score += 100;
    let score: Option<&u32> = scores.get("black");
    println!("Black Score: {:?}", score);

    let score: &mut u32 = scores.entry("blue".to_string()).or_insert(0);
    *score += 100;
    let score: Option<&u32> = scores.get("blue");
    println!("Blue Score: {:?}", score);

    println!("{:#?}", scores);

    let address: String = "0x788f7F2367122e77eeFAE829f65B21701CdF4B74".to_string();
    let amount: u32 = 10012;

    let mut wallet: HashMap<String, u32> = HashMap::new();
    wallet.insert(address.to_string(), amount);
    println!("{:#?}", wallet);
}
