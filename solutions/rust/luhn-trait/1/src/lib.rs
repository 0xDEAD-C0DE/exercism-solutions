pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        let code = self.to_string();
        let code = code.split_whitespace().collect::<String>();
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
