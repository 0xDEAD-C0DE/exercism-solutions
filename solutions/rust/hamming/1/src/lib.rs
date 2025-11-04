
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }
    let dna_tuple = s1.chars().enumerate();
    let dna_to_compare = s2.chars().collect::<Vec<char>>();
    let mut count = 0;
    for (index, dna) in dna_tuple {
        if dna_to_compare.get(index).unwrap() != &dna {
            count += 1;
        }
    }
    Some(count)
    // todo!("What is the Hamming Distance between {s1} and {s2}");
}