use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// The foundational prime number ontology system
/// Based on [0, 1, 2, 3, 5, 7, 11, 13, 17, 23] as semantic primitives
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrimeOntology {
    /// The base prime elements that form the ontological foundation
    pub primes: Vec<u64>,
    /// Semantic mappings of primes to concepts
    pub semantic_mappings: HashMap<u64, SemanticConcept>,
    /// Combinatorial relationships between primes
    pub relationships: HashMap<u64, Vec<PrimeRelation>>,
    /// Dimensional structure for embedding
    pub dimensional_structure: PrimeDimensionalStructure,
}

/// Semantic concepts mapped to prime numbers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SemanticConcept {
    /// 0 - The void, empty set, beginning
    Void,
    /// 1 - Unity, Uranus, eternal one
    Unity,
    /// 2 - Duality, binary, fundamental opposition
    Duality,
    /// 3 - Trinity, synthesis, GCC compilation
    Trinity,
    /// 5 - Quintessence, life force, Rust ecosystem
    Quintessence,
    /// 7 - Completion, spiritual perfection, LLVM IR
    Completion,
    /// 11 - Transcendence, master number, Lean4 proofs
    Transcendence,
    /// 13 - Transformation, death/rebirth, MetaCoq
    Transformation,
    /// 17 - Star, guidance, Haskell types
    Star,
    /// 23 - Cosmic order, universal structure, OCaml modules
    CosmicOrder,
}

/// Relationships between prime numbers in the ontology
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrimeRelation {
    pub target_prime: u64,
    pub relation_type: RelationType,
    pub strength: f32,
    pub semantic_meaning: String,
}

/// Types of relationships between primes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RelationType {
    /// Sequential relationship (e.g., 2 -> 3)
    Sequential,
    /// Multiplicative relationship (e.g., 2 * 3 = 6)
    Multiplicative,
    /// Additive relationship (e.g., 2 + 3 = 5)
    Additive,
    /// Exponential relationship (e.g., 2^3 = 8)
    Exponential,
    /// Semantic similarity in the ontology
    Semantic,
    /// Category theoretical morphism
    Morphism,
}

/// Dimensional structure for prime-based embeddings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrimeDimensionalStructure {
    pub dimension_count: usize,
    pub prime_to_dimension: HashMap<u64, usize>,
    pub embedding_matrix: Vec<Vec<f32>>,
    pub compression_ratio: f32,
}

impl PrimeOntology {
    /// Creates the standard prime number ontology
    pub fn new() -> Self {
        let primes = vec![0, 1, 2, 3, 5, 7, 11, 13, 17, 23];
        let mut semantic_mappings = HashMap::new();

        // Map primes to semantic concepts
        semantic_mappings.insert(0, SemanticConcept::Void);
        semantic_mappings.insert(1, SemanticConcept::Unity);
        semantic_mappings.insert(2, SemanticConcept::Duality);
        semantic_mappings.insert(3, SemanticConcept::Trinity);
        semantic_mappings.insert(5, SemanticConcept::Quintessence);
        semantic_mappings.insert(7, SemanticConcept::Completion);
        semantic_mappings.insert(11, SemanticConcept::Transcendence);
        semantic_mappings.insert(13, SemanticConcept::Transformation);
        semantic_mappings.insert(17, SemanticConcept::Star);
        semantic_mappings.insert(23, SemanticConcept::CosmicOrder);

        let relationships = Self::compute_relationships(&primes);
        let dimensional_structure = Self::create_dimensional_structure(&primes);

        Self {
            primes,
            semantic_mappings,
            relationships,
            dimensional_structure,
        }
    }

    /// Computes relationships between all primes in the ontology
    fn compute_relationships(primes: &[u64]) -> HashMap<u64, Vec<PrimeRelation>> {
        let mut relationships = HashMap::new();

        for &prime in primes {
            let mut relations = Vec::new();

            for &other_prime in primes {
                if prime != other_prime {
                    // Sequential relationship
                    if let Some(index) = primes.iter().position(|&p| p == prime) {
                        if index + 1 < primes.len() && primes[index + 1] == other_prime {
                            relations.push(PrimeRelation {
                                target_prime: other_prime,
                                relation_type: RelationType::Sequential,
                                strength: 1.0,
                                semantic_meaning: "Sequential in prime ontology".to_string(),
                            });
                        }
                    }

                    // Multiplicative relationships
                    if prime != 0 && other_prime != 0 {
                        let product = prime * other_prime;
                        if primes.contains(&product) {
                            relations.push(PrimeRelation {
                                target_prime: other_prime,
                                relation_type: RelationType::Multiplicative,
                                strength: 0.8,
                                semantic_meaning: format!(
                                    "Multiplicative: {} * {} = {}",
                                    prime, other_prime, product
                                ),
                            });
                        }
                    }

                    // Additive relationships
                    let sum = prime + other_prime;
                    if primes.contains(&sum) {
                        relations.push(PrimeRelation {
                            target_prime: other_prime,
                            relation_type: RelationType::Additive,
                            strength: 0.6,
                            semantic_meaning: format!(
                                "Additive: {} + {} = {}",
                                prime, other_prime, sum
                            ),
                        });
                    }
                }
            }

            relationships.insert(prime, relations);
        }

        relationships
    }

    /// Creates dimensional structure for prime-based embeddings
    fn create_dimensional_structure(primes: &[u64]) -> PrimeDimensionalStructure {
        let dimension_count = primes.len();
        let mut prime_to_dimension = HashMap::new();
        let mut embedding_matrix = vec![vec![0.0; dimension_count]; dimension_count];

        // Map each prime to a dimension
        for (i, &prime) in primes.iter().enumerate() {
            prime_to_dimension.insert(prime, i);

            // Create orthogonal basis vectors
            embedding_matrix[i][i] = 1.0;

            // Add semantic correlations
            for (j, &other_prime) in primes.iter().enumerate() {
                if i != j {
                    let correlation = Self::semantic_correlation(prime, other_prime);
                    embedding_matrix[i][j] = correlation;
                }
            }
        }

        PrimeDimensionalStructure {
            dimension_count,
            prime_to_dimension,
            embedding_matrix,
            compression_ratio: 0.618, // Golden ratio compression
        }
    }

    /// Computes semantic correlation between two primes
    fn semantic_correlation(prime1: u64, prime2: u64) -> f32 {
        match (prime1, prime2) {
            // Unity and void have fundamental connection
            (0, 1) | (1, 0) => 0.9,
            // Duality and trinity create synthesis
            (2, 3) | (3, 2) => 0.8,
            // Sequential primes have correlation
            (2, 3) | (3, 5) | (5, 7) | (7, 11) | (11, 13) | (13, 17) | (17, 23) => 0.7,
            // Fibonacci-like relationships
            (2, 5) | (3, 7) | (5, 11) => 0.6,
            // Default weak correlation
            _ => 0.1,
        }
    }

    /// Encodes a concept using the prime ontology
    pub fn encode_concept(&self, concept: &str) -> Vec<f32> {
        let mut encoding = vec![0.0; self.primes.len()];

        // Simple hash-based encoding for now
        let hash = self.hash_concept(concept);
        for (i, &prime) in self.primes.iter().enumerate() {
            let activation = ((hash + prime as u32) % 100) as f32 / 100.0;
            encoding[i] = activation;
        }

        encoding
    }

    /// Decodes a prime-based encoding back to semantic concepts
    pub fn decode_to_concepts(&self, encoding: &[f32]) -> Vec<(SemanticConcept, f32)> {
        let mut concepts = Vec::new();

        for (i, &activation) in encoding.iter().enumerate() {
            if let Some(&prime) = self.primes.get(i) {
                if let Some(concept) = self.semantic_mappings.get(&prime) {
                    concepts.push((concept.clone(), activation));
                }
            }
        }

        // Sort by activation strength
        concepts.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        concepts
    }

    /// Finds the nearest prime in the ontology
    pub fn nearest_prime(&self, value: u64) -> Option<u64> {
        self.primes
            .iter()
            .min_by_key(|&&prime| {
                if prime >= value {
                    prime - value
                } else {
                    value - prime
                }
            })
            .copied()
    }

    /// Generates an address using the prime ontology
    pub fn generate_address(&self, seed: &str) -> Vec<u8> {
        let encoding = self.encode_concept(seed);
        let mut address = Vec::new();

        for (i, &activation) in encoding.iter().enumerate() {
            if let Some(&prime) = self.primes.get(i) {
                let byte_value = ((activation * 255.0) as u8).wrapping_add(prime as u8);
                address.push(byte_value);
            }
        }

        // Extend to 32 bytes for compatibility
        while address.len() < 32 {
            address.push(0);
        }

        address.truncate(32);
        address
    }

    /// Simple hash function for concept encoding
    fn hash_concept(&self, concept: &str) -> u32 {
        let mut hash = 0u32;
        for byte in concept.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u32);
        }
        hash
    }

    /// Validates if a value fits within the prime ontology
    pub fn validate_ontological_fit(&self, value: u64) -> bool {
        // Check if value can be expressed as combinations of our primes
        let mut remaining = value;
        let mut used_primes = HashSet::new();

        for &prime in self.primes.iter().rev() {
            if prime <= remaining && prime > 0 {
                while remaining >= prime {
                    remaining -= prime;
                    used_primes.insert(prime);
                }
            }
        }

        remaining == 0 || used_primes.len() >= 2
    }

    /// Extends the ontology with a new prime
    pub fn extend_with_prime(&mut self, new_prime: u64) {
        if !self.primes.contains(&new_prime) {
            self.primes.push(new_prime);
            self.primes.sort_unstable();

            // Recompute relationships and dimensional structure
            self.relationships = Self::compute_relationships(&self.primes);
            self.dimensional_structure = Self::create_dimensional_structure(&self.primes);
        }
    }
}

impl Default for PrimeOntology {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_ontology_creation() {
        let ontology = PrimeOntology::new();
        assert_eq!(ontology.primes.len(), 10);
        assert!(ontology.semantic_mappings.contains_key(&0));
        assert!(ontology.semantic_mappings.contains_key(&23));
    }

    #[test]
    fn test_concept_encoding() {
        let ontology = PrimeOntology::new();
        let encoding = ontology.encode_concept("test");
        assert_eq!(encoding.len(), 10);
        assert!(encoding.iter().all(|&x| x >= 0.0 && x <= 1.0));
    }

    #[test]
    fn test_address_generation() {
        let ontology = PrimeOntology::new();
        let address = ontology.generate_address("test_seed");
        assert_eq!(address.len(), 32);
    }

    #[test]
    fn test_ontological_validation() {
        let ontology = PrimeOntology::new();
        assert!(ontology.validate_ontological_fit(5));
        assert!(ontology.validate_ontological_fit(8)); // 3 + 5
        assert!(ontology.validate_ontological_fit(15)); // 3 * 5
    }
}
