use std::collections::HashMap;

pub fn nucleotide_counts(s: &str) -> HashMap<char, usize> {

    s.to_string().chars()
        .fold("ATCG".chars().map(|c| (c, 0)).collect::<HashMap<_,_>>(),
            |mut m, c| {
                *m.entry(c).or_insert(0) += 1;
                m
            })
}

pub fn count(c: char, s: &str) -> usize {
    *nucleotide_counts(s).get(&c).unwrap()
}
