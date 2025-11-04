#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    let len = string_digits.len();
    if span > len {
        return Err(Error::SpanTooLong);
    }
    for i in string_digits.chars() {
        if i.is_alphabetic() {
            return Err(Error::InvalidDigit(i));
        }
    }
    let input_slice = string_digits
        .chars()
        .map(|ch| ch.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    let series: Vec<_> = input_slice.windows(span).collect();

    // for i in 0..len - 1 {
    //     if span + i > len {
    //         break;
    //     }
    //     series.push(&input_slice[i..span + i]);
    // }
    let result = series
        .iter()
        .map(|str| str.iter().product())
        .collect::<Vec<u32>>();
    let d = result.iter().max().unwrap();
    Ok(*d as u64)
}
