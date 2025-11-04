pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut set: Vec<u32> = vec![];

    let mut sum;
    for n in  factors {
        for i in 1..limit {
            sum = i * n;
            if sum >= limit {
                break;
            }
            set.push(sum);
        }
    }
    set.sort();
    set.dedup();
    set.iter().sum()
}
