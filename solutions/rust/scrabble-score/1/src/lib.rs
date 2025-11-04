pub fn score(word: &str) -> u64 {
    let word = word.to_ascii_uppercase();
    let mut sum = 0;
    for c in word.chars() {
        match c {
            'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => sum += 1,
            'D' | 'G' => sum += 2,
            'B' | 'C' | 'M' | 'P' => sum += 3,
            'F' | 'H' | 'V' | 'W' | 'Y' => sum += 4,
            'K' => sum += 5,
            'J' | 'X' => sum += 8,
            'Q' | 'Z' => sum += 10,
            _ => sum += 0,
        }
    }
    sum
}