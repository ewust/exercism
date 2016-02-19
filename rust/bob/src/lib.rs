trait Bob {
    fn is_shouting(&self) -> bool;
    fn is_silent(&self) -> bool;
}

impl Bob for str {
    fn is_shouting(&self) -> bool {
        self.chars().all(|c| (!c.is_alphabetic() || c.is_uppercase()))
    }

    fn is_silent(&self) -> bool {
        self.chars().all(|c| !c.is_whitespace())
    }
}


pub fn reply(s: &str) -> &'static str {
    if s.ends_with("?") {
        "Sure."
    } else if s.is_silent() {
        "Fine. Be that way!"
    } else if s.is_shouting() {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}
