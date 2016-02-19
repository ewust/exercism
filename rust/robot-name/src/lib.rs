
extern crate rand;

use rand::{thread_rng, sample};

static ALPHABET: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
static NUMBERS: &'static str = "0123456789";

pub struct Robot {
    name: String
}

fn rand_name() -> String {
    let mut rng = thread_rng();
    let mut out = sample(&mut rng, ALPHABET.chars(), 2);
    let nums = sample(&mut rng, NUMBERS.chars(), 3);
    out.extend(nums.iter().cloned());
    out.iter().cloned().collect::<String>()
}

impl Robot {
    pub fn new() -> Robot {
        Robot { name: rand_name() }
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn reset_name(&mut self) {
        self.name = rand_name();
    }
}

