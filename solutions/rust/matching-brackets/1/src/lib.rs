pub fn brackets_are_balanced(string: &str) -> bool {
        let  str = string.matches(|c| 
            c == '(' || c == '{' || c == '[' || 
            c == ')' || c == '}' || c == ']' ).collect::<String>();
    // println!("{}", str.contains("{{x}}"));
    let mut brackets: String = String::new() ;
    for i in str.chars() {
        if i == '(' {
            brackets.push_str(")");
        }
        else if i == '{' {
            brackets.push_str("}");
        }
        else if i == '[' {
            brackets.push_str("]");
        } 
        else if brackets.is_empty() || brackets.pop() != Some(i) {
            return false;
        }
    } 
    brackets.is_empty()
}


