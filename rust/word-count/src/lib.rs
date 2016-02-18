use std::collections::HashMap;

pub fn word_count(s: &str) -> HashMap<String, u32> {

    let mut m = HashMap::new();

    for word in s.split(|c: char| !(c.is_alphabetic() || c.is_numeric()))
                    .filter(|s| s.len() != 0)
                    .map(|s| s.to_string().to_lowercase()) {

        let counter = m.entry(word).or_insert(0);
        *counter += 1;
    }
    m
}
