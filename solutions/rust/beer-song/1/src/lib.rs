
pub fn verse(n: u32) -> String {
            let bottle = if n > 1 { "bottles"} else { "bottle"};

    match n {
        0 => format!("No more bottles of beer on the wall, no more bottles of beer.\n\
            Go to the store and buy some more, 99 bottles of beer on the wall.\n"),
        1 => format!("{n} {bottle} of beer on the wall, {n} {bottle} of beer.\n\
            Take it down and pass it around, no more bottles of beer on the wall.\n"),
        _ => {
             format!("{n} {bottle} of beer on the wall, {n} {bottle} of beer.\n\
            Take one down and pass it around, {} {} of beer on the wall.\n",
             n - 1, if n - 1 == 1 { "bottle" } else { "bottles"})
        },
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut result: String = String::new();
    let mut s = start;
   
    loop { 
        if s > end {
            result.push_str(&verse(s));
            result.push('\n');
        
        } else {
            result.push_str(&verse(s));
           
            break
        }
        s -= 1;  
    }

    result

}