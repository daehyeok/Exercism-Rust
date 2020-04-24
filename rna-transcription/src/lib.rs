#[derive(Debug, PartialEq)]
pub struct DNA {
    sequence: String,
}

#[derive(Debug, PartialEq)]
pub struct RNA {
    sequence: String,
}

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        let sequence = dna.to_string();
        match sequence.find(|ch| "GCTA".to_string().find(|e| e == ch).is_none()) {
            Some(i) => Err(i),
            _ => Ok(DNA { sequence: sequence }),
        }
    }

    pub fn into_rna(self) -> RNA {
        let cgau = ['C', 'G', 'A', 'U'];
        let gcta = "GCTA".to_string();
        RNA::new(
            &(self
                .sequence
                .chars()
                .map(|c| cgau.get(gcta.find(c).unwrap()).unwrap())
                .collect::<String>()),
        )
        .unwrap()
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        let sequence = rna.to_string();
        match sequence.find(|ch| "CGAU".to_string().find(|e| e == ch).is_none()) {
            Some(i) => Err(i),
            _ => Ok(RNA { sequence: sequence }),
        }
    }
}
