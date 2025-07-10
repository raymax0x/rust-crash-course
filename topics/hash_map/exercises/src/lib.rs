use std::collections::HashMap;

pub fn init(address: String, amount: u32) -> HashMap<String, u32> {
    let mut wallet: HashMap<String, u32> = HashMap::new();
    wallet.insert(address, amount);
    wallet
}
