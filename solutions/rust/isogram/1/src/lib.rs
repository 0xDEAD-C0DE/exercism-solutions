use std::collections::HashSet;
pub fn check(candidate: &str) -> bool {
    let lowercase = candidate.to_lowercase();
    let split = lowercase.split(&['-', ' ']).collect::<String>();
    let mut isogram = HashSet::new();
    split.chars().all(|c| isogram.insert(c))
}