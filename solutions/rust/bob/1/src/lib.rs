pub fn reply(message: &str) -> &str {
   
    let message = message.trim();
    
    match message {
        
        m if m.ends_with('?') => {
            if m.to_uppercase() == message && m.chars().any(char::is_alphabetic) {
                "Calm down, I know what I'm doing!"
            }
            else {
                "Sure."
            }
        },
        m if m.is_empty() => "Fine. Be that way!",
        // m if m.ends_with(|c: char| c.is_whitespace()) => "blank",
        m if m.to_uppercase() == message && m.chars().any(char::is_alphabetic) => {
                "Whoa, chill out!"
        },
        _ => "Whatever.",
    }
}
