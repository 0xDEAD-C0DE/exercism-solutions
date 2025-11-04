pub fn number(user_number: &str) -> Option<String> {
    let mut formatted = user_number.chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    if formatted.len() == 11 && formatted[0] == 1 {
        formatted.remove(0);
    } 

    if formatted.len() == 10 && formatted[0] >= 2 && formatted[3] >= 2 {
        Some(formatted.iter().map(|d| d.to_string()).collect::<String>())
        } else {
            None
        }

}