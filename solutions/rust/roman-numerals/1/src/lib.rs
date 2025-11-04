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
    fn from(num: u32) -> Self {
        let mut roman = String::new();
        let ones = num % 10;
        let tens = (num / 10) % 10;
        let hund = (num / 100) % 10;
        let tho = (num / 1000) % 10;
        // 4 = IV, 9 = IX, 40 = XL, 90 = XC, 400 = CD, 900 = CM
        // 3999
        match tho {
            1..=3 => roman.push_str(&"M".repeat(tho as usize)),
            _ => (),
        }

        match hund {
            1..=3 => roman.push_str(&"C".repeat(hund as usize)),
            4 => roman.push_str("CD"),
            5 => roman.push_str("D"),
            6 => roman.push_str("DC"),
            7 => roman.push_str("DCC"),
            8 => roman.push_str("DCCC"),
            9 => roman.push_str(&"CM"),
            _ => (),
        }

        match tens {
            1..=3 => roman.push_str(&"X".repeat(tens as usize)),
            4 => roman.push_str("XL"),
            5 => roman.push_str("L"),
            6 => roman.push_str("LX"),
            7 => roman.push_str("LXX"),
            8 => roman.push_str("LXXX"),
            9 => roman.push_str(&"XC"),
            _ => (),
        }

        match ones {
            1..=3 => roman.push_str(&"I".repeat(ones as usize)),
            4 => roman.push_str("IV"),
            5 => roman.push_str("V"),
            6 => roman.push_str("VI"),
            7 => roman.push_str("VII"),
            8 => roman.push_str("VIII"),
            9 => roman.push_str(&"IX"),
            _ => (),
        }
        Self { roman }
        
    }
}