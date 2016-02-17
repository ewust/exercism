
#[allow(dead_code)]
fn printvec(chars: &Vec<char>) {
    for c in chars {
        print!("{}", c);
    }
    print!("\n");
}


fn is_anagram(w1: &String, w2: &String) -> bool {
    let mut c1 : Vec<char> = w1.chars().collect();
    let mut c2 : Vec<char> = w2.chars().collect();

    c1.sort();
    c2.sort();
    c1 == c2
}


pub fn anagrams_for<'a>(word: &str, inputs: &[&'a str]) -> Vec<&'a str> {
    let mut out : Vec<&'a str> = vec![];
    let lower_word = word.to_string().to_lowercase();

    for s in inputs.iter() {
        // Is s an anagram of word?
        let lower_s = s.to_string().to_lowercase();

        if lower_s == lower_word {
            continue;
        }

        if is_anagram(&lower_s, &lower_word) {
            println!("I think {} and {} are the same", lower_s, lower_word);
            out.push(s);
        }
    }

    out
}
