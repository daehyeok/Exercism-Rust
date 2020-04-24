use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !"ACGT".chars().any(|ch| ch == nucleotide) {
        return Err(nucleotide);
    }

    match nucleotide_counts(dna) {
        Ok(count) => Ok(*count.get(&nucleotide).unwrap()),
        Err(x) => Err(x),
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut count = "ACGT"
        .chars()
        .map(|c| (c, 0 as usize))
        .collect::<HashMap<char, usize>>();

    for ch in dna.chars() {
        match ch {
            'A' | 'G' | 'C' | 'T' => {
                *(count.entry(ch).or_insert(0)) += 1;
            }
            _ => {
                return Err(ch);
            }
        }
    }
    Ok(count)
}
