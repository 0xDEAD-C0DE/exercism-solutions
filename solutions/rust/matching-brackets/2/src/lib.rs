pub fn brackets_are_balanced(string: &str) -> bool {
    let  str = string.matches(|c| 
        c == '(' || c == '{' || c == '[' || 
        c == ')' || c == '}' || c == ']' ).collect::<String>();
   
    let mut brackets: String = String::new() ;
    for c in str.chars() {
        match c {
            '{' => brackets.push_str("}"),
            '(' => brackets.push_str(")"),
            '[' => brackets.push_str("]"),
            _ => {
                if brackets.is_empty() || brackets.pop() != Some(c) {
                    return false;
                }
            }
        }
    }
    brackets.is_empty()
}


