use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let to_lower = word.to_lowercase();
    let sorted_word = sort_word(&to_lower);

    let result: HashSet<_> = possible_anagrams
        .iter()
        .filter(|w| {
            let lower = w.to_lowercase();
            to_lower != lower && sorted_word == sort_word(&lower)
            
        })
        .cloned()
        .collect();
    result
    
}

fn sort_word(word: &str) -> String {
    let mut word = word.chars().collect::<Vec<char>>();
    word.sort_unstable();
    word.into_iter().collect()
   
}
