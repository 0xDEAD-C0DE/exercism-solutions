pub fn is_valid_isbn(isbn: &str) -> bool {
    if isbn.is_empty() {
        return false
    }
    let mut base = 10;
    let mut sum = 0;
    let isbn = isbn.split(|ch:char| !ch.is_numeric() && ch != 'X').collect::<Vec<&str>>();
    let mut count = 0;

    for str in &isbn {
         
        for d in str.chars() {
            if d.is_digit(10) {
                sum += d.to_digit(10).unwrap() * base;
                // print!("{} ", sum);
                base = base.saturating_sub(1);
            }
            else if d == 'X' || d == 'x' {
                sum += 10 * base;
            } else {
                break;
            }
        count += 1;
        }
    }
    if count != 10 {
        return false;
    }
    sum > 0 && sum % 11 == 0
}
