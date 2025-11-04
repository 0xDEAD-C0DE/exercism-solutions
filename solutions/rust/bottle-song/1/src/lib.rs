pub fn verse(n: u32) -> String {
    let bottle = if n > 1 { "bottles" } else { "bottle" };
    match n {
        0 => format!("There'll be no green bottles hanging on the wall.\n"),
        _ => {
            format!(
                "{} green {bottle} hanging on the wall,\n\
                {} green {bottle} hanging on the wall,\n\
            And if one green bottle should accidentally fall,\n\
            There'll be {} green {} hanging on the wall.\n",
                substitute(n),
                substitute(n),
                substitute(n - 1).to_lowercase(),
                if n - 1 == 1 { "bottle" } else { "bottles" }
            )
        }
    }
}
pub fn substitute(n: u32) -> String {
    match n {
        0 => "no".to_string(),
        1 => "One".to_string(),
        2 => "Two".to_string(),
        3 => "Three".to_string(),
        4 => "Four".to_string(),
        5 => "Five".to_string(),
        6 => "Six".to_string(),
        7 => "Seven".to_string(),
        8 => "Eight".to_string(),
        9 => "Nine".to_string(),
        10 => "Ten".to_string(),

        _ => "".to_string(),
    }
}
pub fn recite(start: u32, end: u32) -> String {
    let mut result: String = String::new();
    let mut s = start;
    let mut count = 1;
    loop {
        result.push_str(&verse(s));
        result.push('\n');

        if count == end {
            // result.push_str(&verse(start));
            break;
        }
        count += 1;
        s -= 1;
    }
    result
}
