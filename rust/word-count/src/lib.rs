use std::collections::HashMap;

pub fn word_count(s: &str) -> HashMap<String, u32> {

    let mut m = HashMap::new();

    for word in s.split(|c: char| !c.is_alphanumeric())
                    .filter(|s| s.len() != 0)
                    .map(|s| s.to_string().to_lowercase()) {

        *m.entry(word).or_insert(0) += 1;
    }
    m
}
