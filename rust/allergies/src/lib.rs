#[allow(dead_code)]
#[derive(Copy, Clone,PartialEq,Debug)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128
}


pub struct allergy {
    score: usize
}

pub fn Allergies(s: usize) -> allergy {
    allergy { score: s }
}
impl allergy {

    pub fn is_allergic_to(&self, allergen : &Allergen) -> bool {
        (self.score & (*allergen as usize)) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {

        let allergens : Vec<Allergen> = vec![Allergen::Eggs,
        Allergen::Peanuts,
        Allergen::Shellfish,
        Allergen::Strawberries,
        Allergen::Tomatoes,
        Allergen::Chocolate,
        Allergen::Pollen,
        Allergen::Cats];

        allergens.iter()
            .filter(|i| (self.score & (**i as usize)) != 0) // What the fuck, rust
            .cloned()
            .collect()
    }
}
