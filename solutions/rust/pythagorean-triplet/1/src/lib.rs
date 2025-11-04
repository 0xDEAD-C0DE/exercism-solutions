use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut triplet = HashSet::new();

    for a in 1..sum / 3 {
        for b in 0..sum / 2 {
            let c = sum - (a + b);

            if a * a + b * b == c * c {
                if a > b {
                    break;
                }
                triplet.insert([a, b, c]);
            }
        }
    }
    triplet
}
