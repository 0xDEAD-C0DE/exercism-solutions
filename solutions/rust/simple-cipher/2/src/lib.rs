use rand::Rng;

pub fn encode(key: &str, s: &str) -> Option<String> {
    // NOTE: Implement the following using zip and map or filter
    let letter = key.contains(|c: char| c.is_numeric() || c.is_uppercase());
    if letter || key.is_empty() {
        return None;
    }
    let mut k = key
        .chars()
        .zip(s.chars())
        .map(|(k, s)| char::from(b'a' + (((k as u8 - b'a') + (s as u8 - b'a')) % 26)).to_string())
        .collect::<String>();

    if s.len() > key.len() {
        k.push_str(s.split_at(k.len()).1);
    }
    Some(k)
    
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    let letter = key.contains(|c: char| c.is_numeric() || c.is_uppercase());
    if letter || key.is_empty() {
        return None;
    }
    let mut k = key
        .chars()
        .zip(s.chars())
        .map(|(k, s)| {
            let res: i8 = (('a' as i8 - k as i8) - ('a' as i8 - s as i8)).rem_euclid(26) as i8;
            char::from(('a' as i8 + res) as u8).to_string()
        })
        .collect::<String>();
    
    // If SUBSTITUTION string is longer, only encode KEY's length and append the remaining characters
    // to the end of the KEY string.
    
    if s.len() > key.len() {
        k.push_str(s.split_at(k.len()).1);
    }
    Some(k)
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut rng = rand::thread_rng();
    let mut key: String = String::new();
    for _ in 0..=100 {
        key.push(rng.gen_range('\u{0062}'..='\u{007A}'));
    }
    let s = encode(&key, s).unwrap();
    (key, s)
    // todo!("Generate random key with only a-z chars and encode {s}. Return tuple (key, encoded s)")
}
