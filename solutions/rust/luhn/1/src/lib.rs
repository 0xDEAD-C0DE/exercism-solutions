pub fn is_valid(code: &str) -> bool {
    let code  = code.split_whitespace()
    .collect::<String>();
    let mut sum = 0;
    let parity = code.len() % 2;

    for (i, c) in code.chars().enumerate() {
        match c.to_digit(10) {
            Some(n) => {
                if i % 2 != parity {
                    sum += n;                        
                }
                else {
                    let mut d = n * 2;
                    if d > 9 {
                        d -= 9;
                    }
                    sum += d;
                }
            }
            None => return false 
        }
    }
    
    code.len() > 1  && sum % 10 == 0 
}