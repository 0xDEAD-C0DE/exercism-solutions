use std::u64;

const INTEGERS: [&str; 7] = [
    "hundred",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
];

const ONES: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const TEENS: [&str; 10] = [
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];
const TENS: [&str; 10] = [
    "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];
pub fn encode(n: u64) -> String {
    match n {
        0..=999 => small_numbers(n),
        1000..=999_999 => {
            if n % 1000 == 0 {
                small_numbers(n / 1000) + " " + INTEGERS[1]
            } else {
                small_numbers(n / 1000) + " " + INTEGERS[1] + " " + &small_numbers(n % 1000)
            }
        }

        1_000_000..=999_999_999 => {
            if n % 1_000_000 == 0 {
                small_numbers(n / 1_000_000) + " " + INTEGERS[2]
            } else {
                small_numbers(n / 1_000_000)
                    + " "
                    + INTEGERS[2]
                    + " "
                    + &small_numbers(n % 1_000_000 / 1000)
                    + " "
                    + INTEGERS[1]
                    + " "
                    + &small_numbers(n % 1000)
            }
        }
        1_000_000_000..=999_999_999_999 => {
            if n % 1_000_000_000 == 0 {
                small_numbers(n / 1_000_000_000) + " " + INTEGERS[3]
            } else {
                small_numbers(n / 1_000_000_000)
                    + " "
                    + INTEGERS[3]
                    + " "
                    + &small_numbers(n / 1_000_000 % 1_000)
                    + " "
                    + INTEGERS[2]
                    + " "
                    + &small_numbers(n % 1_000_000 / 1000)
                    + " "
                    + INTEGERS[1]
                    + " "
                    + &small_numbers(n % 1000)
            }
        }
        1_000_000_000_000..=999_999_999_999_999 => {
            if n % 1_000_000_000_000 == 0 {
                small_numbers(n / 1_000_000_000_000) + " " + INTEGERS[4]
            } else {
                small_numbers(n / 1_000_000_000_000)
                    + " "
                    + INTEGERS[4]
                    + " "
                    + &small_numbers(n / 1_000_000_000 % 1_000)
                    + " "
                    + INTEGERS[3]
                    + " "
                    + &small_numbers(n % 1_000_000_000 / 1_000_000)
                    + " "
                    + INTEGERS[2]
                    + " "
                    + &small_numbers(n % 1_000_000 / 1000)
                    + " "
                    + INTEGERS[1]
                    + " "
                    + &small_numbers(n % 1000)
            }
        }

        1_000_000_000_000_000..=999_999_999_999_999_999 => {
            if n % 1_000_000_000_000_000 == 0 {
                small_numbers(n / 1_000_000_000_000_000) + " " + INTEGERS[5]
            } else {
                small_numbers(n / 1_000_000_000_000_000)
                    + " "
                    + INTEGERS[5]
                    + " "
                    + &small_numbers(n / 1_000_000_000_000 % 1_000)
                    + " "
                    + INTEGERS[4]
                    + " "
                    + &small_numbers(n % 1_000_000_000_000 / 1_000_000_000)
                    + " "
                    + INTEGERS[3]
                    + " "
                    + &small_numbers(n % 1_000_000_000 / 1_000_000)
                    + " "
                    + INTEGERS[2]
                    + " "
                    + &small_numbers(n % 1_000_000 / 1_000)
                    + " "
                    + INTEGERS[1]
                    + " "
                    + &small_numbers(n % 1000)
            }
        }
        1_000_000_000_000_000_000..=u64::MAX => {
            if n % 1_000_000_000_000_000_000 == 0 {
                small_numbers(n / 1_000_000_000_000_000_000) + " " + INTEGERS[4]
            } else {
                small_numbers(n / 1_000_000_000_000_000_000)
                    + " "
                    + INTEGERS[6]
                    + " "
                    + &small_numbers(n / 1_000_000_000_000_000 % 1_000)
                    + " "
                    + INTEGERS[5]
                    + " "
                    + &small_numbers(n % 1_000_000_000_000_000 / 1_000_000_000_000)
                    + " "
                    + INTEGERS[4]
                    + " "
                    + &small_numbers(n % 1_000_000_000_000 / 1_000_000_000)
                    + " "
                    + INTEGERS[3]
                    + " "
                    + &small_numbers(n % 1_000_000_000 / 1_000_000)
                    + " "
                    + INTEGERS[2]
                    + " "
                    + &small_numbers(n % 1_000_000 / 1_000)
                    + " "
                    + INTEGERS[1]
                    + " "
                    + &small_numbers(n % 1000)
            }
        }
        _ => "".to_string(),
    }
}
pub fn small_numbers(n: u64) -> String {
    match n {
        0..=9 => ONES[(n) as usize].to_string(),
        10..=99 => print_tens(n),
        100..=999 => print_hundreds(n),
        _ => "".to_string(),
    }
}
fn print_tens(n: u64) -> String {
    match n {
        0..=9 => ONES[(n) as usize].to_string(),
        10..=19 => TEENS[(n % 10) as usize].to_string(),
        20..=99 => {
            if n % 10 == 0 {
                TENS[(n / 10) as usize].to_string()
            } else {
                TENS[(n / 10) as usize].to_string() + "-" + ONES[(n % 10) as usize]
            }
        }
        _ => "".to_string(),
    }
}
fn print_hundreds(n: u64) -> String {
    if n % 100 == 0 {
        ONES[(n / 100) as usize].to_string() + " " + INTEGERS[0] // + &print_tens(n % 100)
    } else {
        ONES[(n / 100) as usize].to_string() + " " + INTEGERS[0] + " " + &print_tens(n % 100)
    }
}
