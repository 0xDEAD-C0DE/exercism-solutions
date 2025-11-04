pub fn is_pangram(sentence: &str) -> bool {
    // Long code that works too
    // let alphabet = "abcdefghijklmnopqrstuvwxyz";
    // let mut result = -1;
    // for c in alphabet.chars() {
    //     if sentence.to_lowercase().find(c) == None {
    //         result = 1;
    //         break;
    //     }
    // }
    // if result == 1 {
    //     false
    // } else {
    //     true
    // }
     let alphabet = "abcdefghijklmnopqrstuvwxyz";
    alphabet
        .chars()
        .all(|c| sentence.to_lowercase().find(c) != None)
}