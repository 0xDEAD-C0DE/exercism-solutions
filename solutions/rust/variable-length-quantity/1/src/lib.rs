#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];
    for n in values {
        let mut bytes: Vec<u8> = vec![(n & 0x7f) as u8];
        let mut value: u32 = n >> 7;
        while value != 0 {
            bytes.insert(0, (value & 0x7f | 0x80) as u8);
            value >>= 7;
        }
        result.extend(bytes);
    }
    result
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut result: Vec<u32> = vec![];
    let mut value: u32 = 0;

    for b in bytes {
        value <<= 7;
        value |= (b & 0x7f) as u32;
        if b & 0x80 != 0x80 {
            result.push(value);
            value = 0;
        }

        if b & 0x80 == 0x80 && bytes.len() == 1 {
            return Err(Error::IncompleteNumber);
        }
    }
    Ok(result)
}
