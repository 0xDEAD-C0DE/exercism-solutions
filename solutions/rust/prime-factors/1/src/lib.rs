pub fn factors(mut n: u64) -> Vec<u64> {
    let mut factors:Vec<u64> = vec![];
    while n % 2 == 0 {
        factors.push(2);
        n = n / 2;
    }
    let limit = (n as f64).sqrt() as u64;
    for i in (3..=limit).step_by(2) {
        while n % i == 0 {
            factors.push(i);
            n = n / i;
       }
    }
    if n > 2 {
        factors.push(n);
    }
    factors
}
