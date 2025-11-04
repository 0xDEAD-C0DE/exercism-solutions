pub fn raindrops(n: u32) -> String {
    let mut result = String::new();
        match n {
            x if x % 3  == 0 => {
                result.push_str("Pling");
                if x % 5 == 0 {
                    result.push_str("Plang");
                }
                    if x % 7 == 0 {
                        result.push_str("Plong");
                    }
            }
            x if x % 5 == 0 => {
                result.push_str("Plang");
                if x % 7 == 0 {
                    result.push_str("Plong");
                }
            }
            x if x % 7 == 0 => result.push_str("Plong"),
            _ => result.push_str(&n.to_string()),
        }
    result
}

