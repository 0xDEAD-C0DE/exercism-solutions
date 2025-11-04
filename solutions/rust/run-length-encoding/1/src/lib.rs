pub fn encode(source: &str) -> String {
    let mut src = source.chars().peekable();
    let mut count: u32 = 0;
    let mut s: String = String::new();
    while let Some(c) = src.next() {
        count += 1;
        if Some(&c) != src.peek() {
            if count > 1 {
                s.push_str(&count.to_string());
            }
            s.push_str(&c.to_string());
            count = 0;
        }
    }
    s
}

pub fn decode(source: &str) -> String {
    // let s = source
    //     .split(|c: char| c.is_alphabetic())
    //     .collect::<Vec<_>>();
    // let mut digits: Vec<String> = Vec::new();
    // for c in &s {
    //     if *c == "" {
    //         digits.push('1'.to_string());
    //     } else {
    //         digits.push(c.to_string());
    //     }
    // }
    let mut decoded: String = String::new();
    let mut digits: String = String::new();
    // while let Some(d) = src.next() {
    for c in source.chars() {
        if c.is_numeric() {
            digits.push(c);
        } else {
            decoded.push_str(&c.to_string().repeat(digits.parse::<usize>().unwrap_or(1)));
            digits.clear();
        }
    }
    decoded
    // println!("{:?}", decoded);
    // println!("{:?}", digits);
}
