pub fn rotate(input: &str, key: u8) -> String {
    
    const LETTERS: u8 = 26;
    input
        .chars()
        .map(|c| match c {
            'a'..='z' => {
                if c as u8 + key > 'z' as u8 {
                    (c as u8 - (LETTERS.saturating_sub(key))) as char
                } else {
                    (c as u8 + key) as char
                }
            }
            'A'..='Z' => {
                if c as u8 + key > 'Z' as u8 {
                    (c as u8 - (LETTERS.saturating_sub(key))) as char
                } else {
                    (c as u8 + key) as char
                }
            }
            _ => c,
        })
        .collect::<String>()
}