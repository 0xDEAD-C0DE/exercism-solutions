use rand::Rng;

fn mod_exp(base:u64, exponent:u64, modulus:u64) -> u64 {
    let m = modulus as u128;
    let mut b = base as u128 % m;
    let mut e = exponent as u128;

    if m == 1 {
        return 0
    }
    let mut result = 1;
    
    while e > 0 {
        if e % 2 == 1 {
           result = (result * b) % m;
        } 
        e >>= 1;
        b = (b * b) % m;

    }

    result as u64

}
pub fn private_key(p: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..p)
    
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    mod_exp(g, a, p)

}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    mod_exp(b_pub, a, p)
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let limit = (n as f64).sqrt() as u64 + 1;
    for i in 2..=limit {
        if n % i == 0 {
            return false;
        }
    }
    true
}
