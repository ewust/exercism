
fn is_anagram(w1: &String, w2: &String) -> bool {
    let mut c1 : Vec<char> = w1.chars().collect();
    let mut c2 : Vec<char> = w2.chars().collect();

    c1.sort();
    c2.sort();
    c1 == c2 && (w1 != w2)
}

pub fn anagrams_for<'a>(word: &str, inputs: &[&'a str]) -> Vec<&'a str> {
    let lower_word = word.to_string().to_lowercase();

    inputs.iter()
        .filter(|i| is_anagram(&lower_word, &i.to_string().to_lowercase()))
        .cloned()
        .collect()
}
