#[derive(Debug)]
pub struct Allergies {
    allergens: Vec<Allergen>,
}

#[derive(Debug, Clone,PartialEq, Eq)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self {
            allergens: Self::create_allergens(score),
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergens.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.allergens.clone()
    }

    pub fn create_allergens(score: u32) -> Vec<Allergen> {
        let mut allergens = vec![];
        for i in 0..=7 {
            if (score >> i) & 1 == 1 {
                match i {
                    0 => allergens.push(Allergen::Eggs),
                    1 => allergens.push(Allergen::Peanuts),
                    2 => allergens.push(Allergen::Shellfish),
                    3 => allergens.push(Allergen::Strawberries),
                    4 => allergens.push(Allergen::Tomatoes),
                    5 => allergens.push(Allergen::Chocolate),
                    6 => allergens.push(Allergen::Pollen),
                    7 => allergens.push(Allergen::Cats),
                    _ => (),

                }
            }
        }
    allergens
    }
}
