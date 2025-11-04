pub fn number(user_number: &str) -> Option<String> {
    let n = user_number.chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    
    if n.len() == 11 {
       if n[0] != 1 || n[1] < 2 || n[4]  < 2{
        return None
       } 
       else {
        Some(n[1..].iter().map(|d| d.to_string()).collect::<String>())
       }
    }
    else if n.len() == 10 {
        if n[0] < 2 || n[3] < 2 {
            return None
        }
        else {
            Some(n.iter().map(|d| d.to_string()).collect::<String>())
        }
    }
    else if n.len() <= 9 || n.len() > 11 {
        return None
    }
    else {
        Some(n.iter().map(|d| d.to_string()).collect::<String>())
    }
}
