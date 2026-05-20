use std::fmt::{Display, Formatter, Result};

pub struct Roman {
    roman: String,
}

impl Display for Roman {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
        write!(_f, "{}", self.roman)
    }
}

impl From<u32> for Roman {
    fn from(number: u32) -> Self {
        let mut roman = String::new();
        match number / 1000 {
            hund @ 1..=3 => roman.push_str(&"M".repeat(hund as usize)),
            _ => roman.push_str(""),
        }
        match (number % 1000) / 100 {
            hund @ 1..=3 => roman.push_str(&"C".repeat(hund as usize)),
            4 => roman.push_str("CD"),
            5 => roman.push_str("D"),
            hund @ 6..=8 => {
                roman.push_str(&"D");
                roman.push_str(&"C".repeat(hund as usize % 5));
            }
            9 => roman.push_str("CM"),
            _ => roman.push_str(""),
        }
        match (number % 100) / 10 {
            tens @ 1..=3 => roman.push_str(&"X".repeat(tens as usize)),
            4 => roman.push_str("XL"),
            5 => roman.push_str("L"),
            hund @ 6..=8 => {
                roman.push_str(&"L");
                roman.push_str(&"X".repeat(hund as usize % 5));
            }
            9 => roman.push_str("XC"),
            _ => roman.push_str(""),
        }
        match number % 10 {
            ones @ 1..=3 => roman.push_str(&"I".repeat(ones as usize)),
            4 => roman.push_str("IV"),
            5 => roman.push_str("V"),
            ones @ 6..=8 => {
                roman.push_str("V");
                roman.push_str(&"I".repeat(ones as usize % 5));
            }
            9 => roman.push_str("IX"),
            _ => roman.push_str(""),
        }
        Self { roman }
    }
}
