use std::collections::HashMap;

pub fn filter_non_zero(v: Vec<u32>) -> Vec<u32> {
    v.iter().filter(|val| **val > 0).map(|val| *val).collect()
}

pub fn to_string(v: Vec<&str>) -> Vec<String> {
    // v.iter().map(|val : &&str| val.to_string()).collect()
    v.into_iter().map(|val: &str| val.to_string()).collect()
}

pub fn to_hash_map(v: Vec<(String, u32)>) -> HashMap<String, u32> {
    v.into_iter().collect()
}
