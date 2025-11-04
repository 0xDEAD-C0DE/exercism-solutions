use std::str;
pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let info = CodonsInfo::new(make_pairs());
    info.of_rna(rna)
}
fn make_pairs() -> Vec<(&'static str, &'static str)> {
    let grouped = vec![
        ("UUU", "Phenylalanine"),
        ("UUC", "Phenylalanine"),
        ("AUG", "Methionine"),
        ("UAU", "Tyrosine"),
        ("UAC", "Tyrosine"),
        ("UGG", "Tryptophan"),
        ("UGU", "Cysteine"),
        ("UGC", "Cysteine"),
        ("UCU", "Serine"),
        ("UCC", "Serine"),
        ("UCA", "Serine"),
        ("UCG", "Serine"),
        ("UUA", "Leucine"),
        ("UUG", "Leucine"),
        ("UAA", "Stop"),
        ("UGA", "Stop"),
        ("UAG", "Stop"),
    ];
    // grouped.sort_by(|&(_, a), &(_, b)| a.cmp(b));
    grouped
}
pub struct CodonsInfo<'a> {
    pairs: Vec<(&'a str, &'a str)>,
}

impl<'a> CodonsInfo<'a> {
    pub fn new(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
        Self { pairs }
        // todo!("Construct a new CodonsInfo struct from given pairs: {pairs:?}");
    }
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        let r = self
            .pairs
            .iter()
            .filter_map(|(c, p)| if *c == codon { Some(p) } else { None })
            .collect::<Vec<&&str>>();
        match r.first() {
            Some(codon) => Some(codon),
            None => None,
        }
    }

    pub fn of_rna(&self, rna: &'a str) -> Option<Vec<&'a str>> {
        rna.as_bytes()
            .chunks(3)
            .map(|chu| self.name_for(str::from_utf8(chu).unwrap()))
            .take_while(|&chu| chu != Some("Stop"))
            .collect()
    }
}
