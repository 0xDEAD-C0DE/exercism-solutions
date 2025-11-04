#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
    name: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
    name: String,
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let mut n = String::new();
        for (i, c) in dna.chars().enumerate() {
            match c {
                'A' | 'C' | 'G' | 'T' => {
                   n.push(c)
                }
                _ => return Err(i)
            }
        } 
        Ok(Self {
            name: n,
        })
    }

    pub fn into_rna(self) -> Rna {
        let name = &self.name[0..];
        let mut rna = String::new();
        for c in name.chars() {
            match c {
                'A' => rna.push('U'),
                'C' => rna.push('G'),
                'G' => rna.push('C'),
                'T' => rna.push('A'),
                _ => (),

            }
        }
        Rna::new(&rna).unwrap()
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let mut n = String::new();
        for (i, c) in rna.chars().enumerate() {
            match c {
                'U' | 'G' | 'C' | 'A' => {
                   n.push(c)
                }
                _ => return Err(i)
            }
        } 
        Ok(Self {
            name: n,
        })
    }
}

 
