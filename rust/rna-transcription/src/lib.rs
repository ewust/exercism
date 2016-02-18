
#[derive(PartialEq,Debug)]
pub struct RibonucleicAcid {
    strand : String,
}

#[derive(PartialEq,Debug)]
pub struct DeoxyribonucleicAcid {
    strand: String,
}

impl RibonucleicAcid {
    pub fn new(s: &str) -> RibonucleicAcid {
        RibonucleicAcid { strand: s.to_string() }
    }

    pub fn as_ref(&self) -> &str {
        &self.strand
    }
}

impl DeoxyribonucleicAcid {
    pub fn new(s: &str) -> DeoxyribonucleicAcid {
        DeoxyribonucleicAcid { strand: s.to_string() }
    }

    pub fn to_rna(&self) -> RibonucleicAcid {

        RibonucleicAcid { strand:
            self.strand.clone()
                .chars()
                .map(|c| match c {
                    'G' => 'C',
                    'C' => 'G',
                    'T' => 'A',
                    'A' => 'U',
                    _ => '?',
                }).collect()
            }
    }
}
