use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    min_range: u64,
    max_range: u64,
}

impl Palindrome {
    fn new(value: u64, min_range: u64, max_range: u64) -> Self {
        Self {
            value,
            min_range,
            max_range,
        }
    }
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        let mut factors: HashSet<(u64, u64)> = HashSet::new();
        for i in self.min_range..=self.max_range {
            if self.value % i == 0 {
                for j in i..=self.max_range {
                    if j * i == self.value {
                        factors.insert((i, j));
                    }
                }
            }
        }
        factors
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut max_product = 0;
    let mut min_product = u64::MAX;
    if min > max {
        return None;
    }

    for i in (min..=max).rev() {
        for j in (min..=max).rev() {
            let product = j * i;

            if is_palindrome(product) {
                if product > max_product {
                    max_product = product;
                }
                if product < min_product {
                    min_product = product;
                }
            }
        }
    }

    if max_product != 0 && min_product != u64::MAX {
        Some((
            Palindrome::new(min_product, min, max),
            Palindrome::new(max_product, min, max),
        ))
    } else {
        return None;
    }
}
fn is_palindrome(num: u64) -> bool {
    let mut reversed_num = 0;
    let mut temp = num;
    while temp != 0 {
        reversed_num = reversed_num * 10 + temp % 10;
        temp /= 10;
    }

    num == reversed_num
}
