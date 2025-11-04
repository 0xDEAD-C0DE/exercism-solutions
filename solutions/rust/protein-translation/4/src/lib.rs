pub fn translate(rna: &str) -> Option<Vec<&str>> {
    // | Codon              | Amino Acid    |
    // | :----------------- | :------------ |
    // | AUG                | Methionine    |
    // | UUU, UUC           | Phenylalanine |
    // | UUA, UUG           | Leucine       |
    // | UCU, UCC, UCA, UCG | Serine        |
    // | UAU, UAC           | Tyrosine      |
    // | UGU, UGC           | Cysteine      |
    // | UGG                | Tryptophan    |
    // | UAA, UAG, UGA      | STOP          |
    //
    let mut proteins = vec![];
    for codon in rna.as_bytes().chunks(3) {
        unsafe {
            match std::str::from_utf8_unchecked(codon) {
                "AUG" => proteins.push("Methionine"),
                "UUU" | "UUC" => proteins.push("Phenylalanine"),
                "UUA" | "UUG" => proteins.push("Leucine"),
                "UCU" | "UCC" | "UCA" | "UCG" => proteins.push("Serine"),
                "UAU" | "UAC" => proteins.push("Tyrosine"),
                "UGU" | "UGC" => proteins.push("Cysteine"),
                "UGG" => proteins.push("Tryptophan"),
                "UAA" | "UAG" | "UGA" => break,
                _ => return None,
            }
        }
    }
    Some(proteins)
}
