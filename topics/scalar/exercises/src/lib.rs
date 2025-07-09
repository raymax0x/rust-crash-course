pub fn eq(a: char, b: char) -> bool {
    a == b
}

pub fn add(x: f32, y: f32, z: f32) -> f32 {
    x + y + z
}

pub fn cast(x: u8, y: i8, z: f32) -> f32 {
    let first_value = x as f32;
    let second_value = y as f32;
    let sum: f32 = first_value + second_value + z;

    sum
}
