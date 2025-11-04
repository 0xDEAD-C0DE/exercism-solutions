use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
   h.iter()
        .map(|(dig, vec)| vec.iter().map(|num| (num.to_ascii_lowercase(), *dig)))
        .flatten()
        .collect()
}