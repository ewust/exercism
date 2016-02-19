

pub fn reply(s: &str) -> &'static str {
    if s.ends_with("?") {
        "Sure."
    } else if s.is_empty() {
        "Fine. Be that way!"
    } else if s.chars().fold(true, |mut acc, c| {
      acc &= (!c.is_alphabetic() || c.is_uppercase());
      acc }) {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}
