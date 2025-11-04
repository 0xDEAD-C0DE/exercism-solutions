use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut w = HashMap::new();
    let words = words.to_lowercase();
    let formatted = words.split_whitespace()
        .map(|s| s.split(
                &['!', '"', '#', '$', '%', '&',
                 '(', ')', '*', '+', ',', '-', '.', '/',
                 ':', ';', '<', '=', '>', '?', '@', 
                 '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~',
                 ]).map(|s| s.trim_matches(|c: char| c.is_ascii_punctuation())).collect::<Vec<&str>>()).collect::<Vec<_>>();
        // c == '.' || c == '-' || c == ':').collect::<Vec<&str>>();

    for s in formatted {
        for word in s {
            if word == "" {
                continue;
            }
           let count = w.entry(word.to_string()).or_insert(0);
           *count += 1; 
        }
    }
    w
}
