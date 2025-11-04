
pub fn encrypt(input: &str) -> String {
    let s = normalize(input);
    let col = (s.len() as f32).sqrt().ceil() as u32;
     
    (0..col as usize)
        .map(|i| 
             s.chunks(col as usize)
             .filter_map(|chu| chu.get(i).or(Some(&' ')))
            .collect::<String>()
        ).collect::<Vec<String>>()
        .join(" ")
    
}

pub fn normalize(s: &str) -> Vec<char> {
    s.to_ascii_lowercase()
    .chars()
    .filter(|c| c.is_alphanumeric())
    .collect::<Vec<char>>()
}
