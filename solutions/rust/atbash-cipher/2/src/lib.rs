/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
  
    let n = plain
        .to_ascii_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .collect::<String>();
    let n = n
        .as_bytes()
        .chunks(5)
        .map(|chu| {
            chu.iter()
                .map(|b| match b {
                    b'a'..=b'z' => char::from((b'a'+ b'z') - *b as u8).to_string(),
                    b'0'..=b'9' => char::from(*b).to_string(),
                    _ => "".to_string(),
                })
                .collect::<Vec<String>>()
                .join("")
        })
        .collect::<Vec<String>>()
        .join(" ");
    n
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    let n = cipher
        .to_ascii_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .collect::<String>();
    let n = n
        .to_lowercase()
        .chars()
        .map(|c| match c {
            'a'..='z' => ((b'a' + b'z') - c as u8) as char,
            _ => c,
        })
        .collect::<String>();
    n
    // todo!("Decoding of {cipher:?} in Atbash cipher.");
}
