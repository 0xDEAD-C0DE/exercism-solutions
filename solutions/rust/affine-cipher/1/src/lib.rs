/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.

#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

const M: i32 = 26;
/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if gcd(a, M) != 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }
    let new = plaintext
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| match c {
            'a'..='z' => {
                let i = c as i32 - 'a' as i32;
                char::from(b'a' + (((a * i) + b as i32) % M) as u8)
            }
            _ => c,
        })
        .collect::<Vec<char>>();
    let new: Vec<&[char]> = new.chunks(5).collect();
    let new = new.join(&' ').iter().collect::<String>();
    // println!("{:?}", new);
    Ok(new)

    // E(x) = (ai + b) mod m
    // i is the letter's index from 0 to the length of the alphabet - 1
    // m is the length of the alphabet. For the Roman alphabet m is 26.
    // a and b are integers which make the encryption key
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    if gcd(a, M) != 1 {
        return Err(AffineCipherError::NotCoprime(a));
    }

    let new = ciphertext
        .to_ascii_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|y| match y {
            'a'..='z' => {
                let i = y as i32 - b'a' as i32;
                // println!("{}", i);
                let mmi = mmi(a, M);
                char::from(b'a' + (mmi * (i - b)).rem_euclid(M) as u8).to_string()
            }
            _ => y.to_string(),
        })
        .collect::<String>();

    Ok(new)
    // D(y) = (a^-1)(y - b) mod m
    // y is the numeric value of an encrypted letter, i.e., y = E(x)
    // it is important to note that a^-1 is the modular multiplicative inverse (MMI) of a mod m
    // the modular multiplicative inverse only exists if a and m are coprime.
    // The MMI of a is x such that the remainder after dividing ax by m is 1:
    // todo!("Decode {ciphertext} with the key ({a}, {b})");
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
fn mmi(a: i32, b: i32) -> i32 {
    let mut mmi = 1;
    while (a * mmi) % b != 1 {
        mmi += 1;
    }
    mmi
}