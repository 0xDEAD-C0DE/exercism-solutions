
pub fn actions(n: u8) -> Vec<&'static str> {
    let b = (0..5).rev().map(|b| ((n >> b) & 1)).collect::<Vec<u8>>();
    
    let mut r = b
        .iter()
        .rev()
        .enumerate()
        .map(|(i, d)| match (i, d) {
            (0, 1) => "wink",
            (1, 1) => "double blink",
            (2, 1) => "close your eyes",
            (3, 1) => "jump",
            _ => "",
        })
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();
    if ((n >> 4) & 1) == 1 {
        
        r.reverse()
    }
    r
}

