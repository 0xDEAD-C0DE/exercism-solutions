use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    match nucleotide_counts(dna) {
        Ok(result) =>  {
            match result.get(&nucleotide) {
                Some(n) => Ok(*n),
                None => Err(nucleotide), 
            }
        },
        Err(e) => return Err(e)
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut dna_sequence = HashMap::new(); 
    dna_sequence.insert('A', 0);
    dna_sequence.insert('C', 0);
    dna_sequence.insert('G', 0);
    dna_sequence.insert('T', 0);
        for n in dna.chars() {
            match n {
                'A' | 'G' | 'C' | 'T' =>  {
                    let count = dna_sequence.entry(n).or_insert(0);
                    *count += 1;

                 },
                 _ => return Err(n)
            }

        }

    Ok(dna_sequence)
}