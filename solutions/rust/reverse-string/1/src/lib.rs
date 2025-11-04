pub fn reverse(input: &str) -> String {
    let mut char = input.chars().collect::<Vec<_>>();
    char.reverse();
    let mut str = String::new();
    for c in char {
        str.push(c);
    }
    str
}
