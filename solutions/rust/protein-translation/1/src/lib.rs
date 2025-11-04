pub struct CodonsInfo<'a> {
    pairs: Vec<(&'a str, &'a str)>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        let r = self
            .pairs
            .iter()
            .filter_map(|(c, p)| if *c == codon { Some(p) } else { None })
            .collect::<Vec<&&str>>();
        match r.get(0) {
            Some(codon) => Some(codon),
            None => None,
        }
    }

    pub fn of_rna(&self, rna: &'a str) -> Option<Vec<&'a str>> {
       rna.as_bytes()
            .chunks(3)
            .map(|chu| self.name_for(std::str::from_utf8(chu).unwrap()))
            .take_while(|&chu| chu != Some("stop codon"))
            .collect()
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo { pairs }
    // todo!("Construct a new CodonsInfo struct from given pairs: {pairs:?}");
}