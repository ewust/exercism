use std::collections::HashMap;

pub fn word_count<'a>(s: &'a str) -> HashMap<String, u32> {

    let mut m: HashMap<String, u32> = HashMap::new();

    for word in s.split(|c: char| !(c.is_alphabetic() || c.is_numeric()))
                    .filter(|s| s.len() != 0)
                    .map(|s| s.to_string()) {

        match m.get(&word) {
            Some(&count) => m.insert(word, count + 1),
            None => m.insert(word, 1)
        }
    }
    m
}
