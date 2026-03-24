// Core Symbiotic Relationship Engine

// Define the structures and functionalities of the symbiotic relationship

pub struct SymbioticRelationship {
    pub organism_a: String,
    pub organism_b: String,
    pub relationship_type: String,
}

impl SymbioticRelationship {
    pub fn new(organism_a: &str, organism_b: &str, relationship_type: &str) -> Self {
        SymbioticRelationship {
            organism_a: organism_a.to_string(),
            organism_b: organism_b.to_string(),
            relationship_type: relationship_type.to_string(),
        }
    }

    pub fn display_relationship(&self) {
        println!("{} and {} have a {} relationship.", self.organism_a, self.organism_b, self.relationship_type);
    }
}

// Example usage
fn main() {
    let mutualism = SymbioticRelationship::new("Bee", "Flower", "Mutualism");
    mutualism.display_relationship();
}