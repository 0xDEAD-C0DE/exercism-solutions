use std::collections::HashSet;
pub fn check(candidate: &str) -> bool {
    let mut isogram = HashSet::new();
    candidate.to_lowercase().chars().filter(|c| c.is_alphabetic()).all(|c| isogram.insert(c))
}