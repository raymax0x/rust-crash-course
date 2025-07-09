pub fn hello() -> String {
    "Hello Rust".to_string()
}

pub fn greet(name: &str) -> String {
    let greet: String = format!("Hello {name}");
    greet
}

pub fn append(mut s: String) -> String {
    let append_string: String = s + "!";
    append_string
}
