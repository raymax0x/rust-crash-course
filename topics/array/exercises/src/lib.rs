pub fn zeros() -> [u32; 100] {
    let x: [u32; 100] = [0; 100];
    x
}

pub fn first_3(s: &[u32]) -> &[u32] {
    &s[..3]
}

pub fn last_3(s: &[u32]) -> &[u32] {
    let n = s.len();
    &s[n - 3..]
}
