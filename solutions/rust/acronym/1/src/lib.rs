pub fn abbreviate(phrase: &str) -> String {
    let mut acronym: String = String::new();
    let str:Vec<_>  = phrase.split( &[' ', '-', '_'] ).collect();
    for slice in &str {
        if slice.contains(&"HyperText")  {
            for c in slice.chars() {
                if c.is_uppercase() {
                    acronym.push(c);
                }
            }
        }
        else {
            match slice.chars().nth(0) {
                Some(c) => acronym.push(c),
                None => ()
            }
        }
    }

    acronym.to_uppercase()

}
