//! Braid elements with full group operations.
//!
//! A braid is an element of the braid group B_n, represented as a word
//! in the Artin generators. This module provides composition, inverse,
//! and identity operations.

use crate::generator::Generator;
use crate::reduction;
use crate::word::BraidWord;

/// A braid in B_n (the braid group on n strands).
///
/// # Examples
///
/// ```
/// use braid_group_rs::braid::Braid;
/// use braid_group_rs::generator::Generator;
///
/// let b = Braid::from_generators(3, &[Generator::new(1), Generator::new(2)]);
/// assert_eq!(b.n_strands(), 3);
/// assert_eq!(b.word_len(), 2);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Braid {
    /// Number of strands (n in B_n).
    n: usize,
    /// The braid word.
    word: BraidWord,
}

impl Braid {
    /// Create the identity braid in B_n (empty word).
    pub fn identity(n: usize) -> Self {
        assert!(n >= 2, "Braid group needs at least 2 strands");
        Self {
            n,
            word: BraidWord::empty(),
        }
    }

    /// Create a braid from a list of generators in B_n.
    pub fn from_generators(n: usize, gens: &[Generator]) -> Self {
        assert!(n >= 2);
        for g in gens {
            assert!(
                g.abs_index() < n as i32,
                "Generator index {} exceeds strand count {}",
                g.abs_index(),
                n - 1
            );
        }
        Self {
            n,
            word: BraidWord::from_vec(gens.to_vec()),
        }
    }

    /// Create a braid from raw indices in B_n.
    pub fn from_indices(n: usize, indices: &[i32]) -> Self {
        assert!(n >= 2);
        for &i in indices {
            assert!(i != 0, "Generator index cannot be 0");
            assert!(
                i.abs() < n as i32,
                "Generator index {} exceeds strand count {}",
                i.abs(),
                n - 1
            );
        }
        Self {
            n,
            word: BraidWord::from_indices(indices),
        }
    }

    /// Get the number of strands.
    pub fn n_strands(&self) -> usize {
        self.n
    }

    /// Get the length of the braid word.
    pub fn word_len(&self) -> usize {
        self.word.len()
    }

    /// Check if this is the identity braid (empty word after reduction).
    pub fn is_identity(&self) -> bool {
        let reduced = self.reduced();
        reduced.word.is_empty()
    }

    /// Check if the unreduced word is empty (identity).
    pub fn is_reduced_identity(&self) -> bool {
        self.word.is_empty()
    }

    /// Compose (concatenate) two braids: self · other.
    ///
    /// Both braids must be in the same group B_n.
    ///
    /// # Panics
    ///
    /// Panics if the braids have different strand counts.
    pub fn compose(&self, other: &Braid) -> Braid {
        assert_eq!(self.n, other.n, "Cannot compose braids from different groups");
        let mut word = self.word.clone();
        word.concat(&other.word);
        Braid { n: self.n, word }
    }

    /// Compute the inverse braid.
    ///
    /// The inverse reverses the word and negates each generator.
    pub fn inverse(&self) -> Braid {
        let reversed = self.word.reversed();
        let inverted: Vec<Generator> = reversed.iter().map(|g| g.inverse()).collect();
        Braid {
            n: self.n,
            word: BraidWord::from_vec(inverted),
        }
    }

    /// Reduce the braid word by canceling adjacent inverse pairs.
    ///
    /// This performs free reduction: σ_i · σ_i^{-1} → ε and σ_i^{-1} · σ_i → ε.
    pub fn reduced(&self) -> Braid {
        let reduced_word = reduction::free_reduce(&self.word);
        Braid {
            n: self.n,
            word: reduced_word,
        }
    }

    /// Apply braid relations: σ_i σ_j σ_i = σ_j σ_i σ_j when |i-j| = 1.
    ///
    /// Returns a simplified (but not necessarily fully reduced) braid.
    pub fn simplify(&self) -> Braid {
        let mut word = reduction::free_reduce(&self.word);
        word = reduction::apply_braid_relations(&word);
        Braid { n: self.n, word }
    }

    /// Get the braid word.
    pub fn word(&self) -> &BraidWord {
        &self.word
    }

    /// Get the permutation induced by this braid on the strands.
    ///
    /// Returns a permutation of {0, 1, ..., n-1}.
    pub fn permutation(&self) -> Vec<usize> {
        let mut perm: Vec<usize> = (0..self.n).collect();
        for g in self.word.iter() {
            let i = g.abs_index() as usize;
            if g.is_positive() {
                // σ_i: strand i goes over strand i+1
                perm.swap(i - 1, i);
            } else {
                // σ_i^{-1}: strand i+1 goes over strand i
                perm.swap(i - 1, i);
            }
        }
        perm
    }

    /// Check if the induced permutation is the identity.
    pub fn is_pure(&self) -> bool {
        let perm = self.permutation();
        perm.iter().enumerate().all(|(i, &p)| i == p)
    }
}

impl std::fmt::Display for Braid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "B_{}: {}", self.n, self.word)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_braid() {
        let id = Braid::identity(3);
        assert_eq!(id.n_strands(), 3);
        assert!(id.is_reduced_identity());
        assert!(id.is_identity());
    }

    #[test]
    fn test_from_indices() {
        let b = Braid::from_indices(4, &[1, 2, -1]);
        assert_eq!(b.word_len(), 3);
        assert_eq!(b.n_strands(), 4);
    }

    #[test]
    fn test_compose_identity() {
        let id = Braid::identity(3);
        let b = Braid::from_indices(3, &[1, 2]);
        let composed = b.compose(&id);
        assert_eq!(composed.word().to_indices(), vec![1, 2]);
    }

    #[test]
    fn test_compose_associativity() {
        let a = Braid::from_indices(3, &[1]);
        let b = Braid::from_indices(3, &[2]);
        let c = Braid::from_indices(3, &[1]);
        let ab_c = a.compose(&b).compose(&c);
        let a_bc = a.compose(&b.compose(&c));
        assert_eq!(ab_c.word().to_indices(), a_bc.word().to_indices());
    }

    #[test]
    fn test_inverse_cancels() {
        let b = Braid::from_indices(3, &[1, 2, -1]);
        let inv = b.inverse();
        let product = b.compose(&inv);
        let reduced = product.reduced();
        assert!(reduced.is_reduced_identity(), "b * b^-1 should reduce to identity");
    }

    #[test]
    fn test_inverse_of_identity() {
        let id = Braid::identity(3);
        let inv = id.inverse();
        assert!(inv.is_reduced_identity());
    }

    #[test]
    fn test_inverse_of_inverse() {
        let b = Braid::from_indices(4, &[1, -2, 3]);
        let inv_inv = b.inverse().inverse();
        assert_eq!(inv_inv.word().to_indices(), b.word().to_indices());
    }

    #[test]
    fn test_free_reduction() {
        let b = Braid::from_indices(3, &[1, -1, 2]);
        let reduced = b.reduced();
        assert_eq!(reduced.word().to_indices(), vec![2]);
    }

    #[test]
    fn test_free_reduction_all_cancel() {
        let b = Braid::from_indices(3, &[1, 2, -2, -1]);
        let reduced = b.reduced();
        assert!(reduced.is_reduced_identity());
    }

    #[test]
    fn test_free_reduction_nested() {
        let b = Braid::from_indices(3, &[1, -1, 1, -1]);
        let reduced = b.reduced();
        assert!(reduced.is_reduced_identity());
    }

    #[test]
    fn test_permutation_sigma1() {
        let b = Braid::from_indices(3, &[1]);
        let perm = b.permutation();
        assert_eq!(perm, vec![1, 0, 2]);
    }

    #[test]
    fn test_permutation_identity() {
        let id = Braid::identity(3);
        let perm = id.permutation();
        assert_eq!(perm, vec![0, 1, 2]);
    }

    #[test]
    fn test_display() {
        let b = Braid::from_indices(3, &[1, -2]);
        let s = format!("{b}");
        assert!(s.contains("B_3"));
    }

    #[test]
    fn test_pure_braid() {
        // σ_1^2 is a pure braid (trivial permutation)
        let b = Braid::from_indices(3, &[1, 1]);
        assert!(b.is_pure());
    }

    #[test]
    fn test_non_pure_braid() {
        let b = Braid::from_indices(3, &[1]);
        assert!(!b.is_pure());
    }

    #[test]
    #[should_panic(expected = "at least 2 strands")]
    fn test_too_few_strands() {
        Braid::identity(1);
    }

    #[test]
    #[should_panic]
    fn test_generator_out_of_range() {
        Braid::from_indices(3, &[3]); // σ_3 doesn't exist in B_3
    }
}
