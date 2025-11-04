pub fn collatz(n: u64) -> Option<u64> {
    let mut steps = 0;
    let mut result = n;
    if n < 1 || n > u64::MAX {
        return None;
    }

    while result != 1 {
        if result >= u64::MAX {
            return None;
        }
        if result % 2 == 0 {
            result /= 2;
        } else {
            result = 3u64.saturating_mul(result).saturating_add(1);
        }
    steps += 1;

    }

    Some(steps)
}
