pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }

    let mut count = 0;
    let mut i = 2;
    while count != n {
        i += 1;
        if is_prime(i) {
            // println!("{:?}", i);
            count += 1;
        }
    }
    i
}

pub fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    let limit = (n as f32).sqrt() as u32 + 1; 
    for i in 2..=limit  {
        if n % i == 0 {
            return false;
        }
    }
    true
}
