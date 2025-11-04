use std::fmt;

pub struct Luhn<T> {
    num: T,
}

impl<T: fmt::Display> Luhn<T> {
    pub fn is_valid(&self) -> bool {
        let code = self.num.to_string().split_whitespace().collect::<String>();
        let mut sum = 0;
        let parity = code.len() % 2;

        for (i, c) in code.chars().enumerate() {
            match c.to_digit(10) {
                Some(n) => {
                    if i % 2 != parity {
                        sum += n;
                    } else {
                        let mut d = n * 2;
                        if d > 9 {
                            d -= 9;
                        }
                        sum += d;
                    }
                }
                None => return false,
            }
        }
        code.len() > 1 && sum % 10 == 0
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
// impl<'a> From<&'a str> for Luhn {
impl<T> From<T> for Luhn<T> {
    fn from(input: T) -> Self {
        Self { num: input }
    }
}