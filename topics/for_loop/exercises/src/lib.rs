pub fn sum(nums: Vec<i32>) -> i32 {
    let mut total: i32 = 0;
    for m in nums {
        total += m;
    }
    total
}

pub fn fill(i: u32, n: usize) -> Vec<u32> {
    let mut v: Vec<u32> = vec![];

    for _ in 0..n {
        v.push(i);
    }

    v
}
