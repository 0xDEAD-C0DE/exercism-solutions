use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut new: BTreeMap<char, i32> = BTreeMap::new();
    for (value, vec) in h {
        for l in vec {
            new.insert(l.to_ascii_lowercase(), *value);
        }
    }
    new
    // todo!("How will you transform the tree {h:?}?")
}