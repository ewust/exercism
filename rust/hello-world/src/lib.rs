
pub fn hello(name: Option<&str>) -> String {
    match name {
        Some(p) => ("Hello, ".to_string() + p).to_string() + "!",
        None => "Hello, World!".to_string()
    }
}
