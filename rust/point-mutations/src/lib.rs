

pub fn hamming_distance(a: &str, b: &str) -> Result<usize, &'static str> {
    if a.len() != b.len() {
        return Err("inputs of different length");
    }

    Ok(a.to_string()
        .chars()
        .zip(b.to_string().chars())
        .fold(0, |mut diff, (x, y)| {
        if x != y {
            diff += 1;
        }
        diff
    }))
}
