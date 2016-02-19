


pub fn raindrops(n: usize) -> String {
    let mut out = "".to_string();
    if (n % 3) == 0 {
        out.push_str("Pling");
    }
    if (n % 5) == 0 {
        out.push_str("Plang");
    }
    if (n % 7) == 0 {
        out.push_str("Plong");
    }
    if out.is_empty() {
        out.push_str(&n.to_string());
    }
    out
}
