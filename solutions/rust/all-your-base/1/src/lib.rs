#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    for n in number {
        if from_base == *n {
            return Err(Error::InvalidDigit(*n));
        }
    }
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }

    let mut result: Vec<u32> = vec![];
    let mut base_ten = to_base_ten(number, from_base);

    if base_ten == 0 {
        return Ok(vec![0]);
    }

    while base_ten != 0 {
        let r = base_ten % to_base;
        result.push(r);
        base_ten /= to_base;
    }
    let result = result.iter().rev().map(|d| *d).collect();
    Ok(result)

    // todo!("Convert {number:?} from base {from_base} to base {to_base}")
}

pub fn to_base_ten(number: &[u32], from_base: u32) -> u32 {
    number
        .iter()
        .rev()
        .enumerate()
        .map(|(i, val)| from_base.pow(i as u32) * val)
        .sum()
}
