
use std::collections::BTreeMap;

pub fn transform(old: &BTreeMap<i32, Vec<String>>) -> BTreeMap<String, i32> {
    old.iter().fold(BTreeMap::new(), |mut acc, (&n, ref v)| {
        // Weird that you can't just do v.iter().map()...scoping weirdness?
        for s in v.iter() {
            acc.insert(s.to_lowercase().clone(), n);
        }
        acc
    })
}
