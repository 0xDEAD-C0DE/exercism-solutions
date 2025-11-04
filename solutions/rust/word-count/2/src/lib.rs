use std::collections::HashMap;
/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut w = HashMap::new();
    let words = words.to_lowercase();

    let formatted = words.split(|c:char| !c.is_alphanumeric() && c != '\'')
                 .map(|s| s.trim_matches(|c: char| c.is_ascii_punctuation())).collect::<Vec<&str>>();

    for s in formatted {
        if s == "" {
            continue;
        }
        let count = w.entry(s.to_string()).or_insert(0);
        *count += 1; 
    }
    w
}
