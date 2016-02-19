use std::collections::HashMap;

pub struct codon<'a> {
    names: HashMap<&'a str, &'a str>
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> codon<'a> {
    codon { names: pairs.iter().cloned().collect::<HashMap<_,_>>() }
}

impl <'a>codon<'a> {
    pub fn name_for(&self, s: &'a str) -> Result<&'a str, &'static str> {

        let instance: String = s.chars().map(|c| match c {
            'Y' => 'C',
            'R' => 'A',
            'K' => 'G',
            'M' => 'A',
            'S' => 'C',
            'W' => 'A',
            'B' => 'C',
            'D' => 'A',
            'H' => 'A',
            'V' => 'A',
            'N' => 'A',
            _ => c
        }).collect::<String>();
        match self.names.get(&instance.as_ref()) {
            Some(name) => Ok(name),
            None => Err("Not found")
        }
    }
}
