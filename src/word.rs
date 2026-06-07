//! Braid word representation and manipulation.
//!
//! A braid word is a sequence of Artin generators. This module provides
//! operations on braid words including concatenation, length, and indexing.

use crate::generator::Generator;

/// A braid word: a sequence of Artin generators.
///
/// # Examples
///
/// ```
/// use braid_group_rs::word::BraidWord;
/// use braid_group_rs::generator::Generator;
///
/// let word = BraidWord::from_vec(vec![Generator::new(1), Generator::new(-2)]);
/// assert_eq!(word.len(), 2);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BraidWord {
    /// The sequence of generators.
    generators: Vec<Generator>,
}

impl BraidWord {
    /// Create an empty braid word (identity element).
    pub fn empty() -> Self {
        Self { generators: Vec::new() }
    }

    /// Create a braid word from a vector of generators.
    pub fn from_vec(generators: Vec<Generator>) -> Self {
        Self { generators }
    }

    /// Create a braid word from a slice of indices.
    ///
    /// Positive indices represent σ_i, negative represent σ_i^{-1}.
    pub fn from_indices(indices: &[i32]) -> Self {
        Self {
            generators: indices.iter().map(|&i| Generator::new(i)).collect(),
        }
    }

    /// Get the length of the braid word.
    pub fn len(&self) -> usize {
        self.generators.len()
    }

    /// Check if the braid word is empty (identity).
    pub fn is_empty(&self) -> bool {
        self.generators.is_empty()
    }

    /// Get the generator at position `i`.
    pub fn get(&self, i: usize) -> Option<Generator> {
        self.generators.get(i).copied()
    }

    /// Append a generator to the end of the word.
    pub fn push(&mut self, g: Generator) {
        self.generators.push(g);
    }

    /// Concatenate another braid word to this one.
    pub fn concat(&mut self, other: &BraidWord) {
        self.generators.extend_from_slice(&other.generators);
    }

    /// Get a slice of the generators.
    pub fn as_slice(&self) -> &[Generator] {
        &self.generators
    }

    /// Iterate over the generators.
    pub fn iter(&self) -> impl Iterator<Item = &Generator> {
        self.generators.iter()
    }

    /// Reverse the braid word (for inverse computation).
    pub fn reversed(&self) -> BraidWord {
        Self {
            generators: self.generators.iter().rev().copied().collect(),
        }
    }

    /// Convert to a vector of raw indices.
    pub fn to_indices(&self) -> Vec<i32> {
        self.generators.iter().map(|g| g.index()).collect()
    }
}

impl std::fmt::Display for BraidWord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.generators.is_empty() {
            write!(f, "ε")?;
        } else {
            for (i, g) in self.generators.iter().enumerate() {
                if i > 0 {
                    write!(f, "·")?;
                }
                write!(f, "{g}")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_word() {
        let w = BraidWord::empty();
        assert!(w.is_empty());
        assert_eq!(w.len(), 0);
    }

    #[test]
    fn test_from_vec() {
        let w = BraidWord::from_vec(vec![Generator::new(1), Generator::new(2)]);
        assert_eq!(w.len(), 2);
    }

    #[test]
    fn test_from_indices() {
        let w = BraidWord::from_indices(&[1, -2, 3]);
        assert_eq!(w.len(), 3);
        assert_eq!(w.get(0).unwrap().index(), 1);
        assert_eq!(w.get(1).unwrap().index(), -2);
    }

    #[test]
    fn test_push() {
        let mut w = BraidWord::empty();
        w.push(Generator::new(1));
        assert_eq!(w.len(), 1);
    }

    #[test]
    fn test_concat() {
        let mut w1 = BraidWord::from_indices(&[1, 2]);
        let w2 = BraidWord::from_indices(&[3]);
        w1.concat(&w2);
        assert_eq!(w1.to_indices(), vec![1, 2, 3]);
    }

    #[test]
    fn test_reversed() {
        let w = BraidWord::from_indices(&[1, 2, 3]);
        let r = w.reversed();
        assert_eq!(r.to_indices(), vec![3, 2, 1]);
    }

    #[test]
    fn test_to_indices() {
        let w = BraidWord::from_indices(&[1, -2, 3]);
        assert_eq!(w.to_indices(), vec![1, -2, 3]);
    }

    #[test]
    fn test_get_out_of_bounds() {
        let w = BraidWord::from_indices(&[1]);
        assert!(w.get(5).is_none());
    }

    #[test]
    fn test_display_empty() {
        assert_eq!(format!("{}", BraidWord::empty()), "ε");
    }

    #[test]
    fn test_display_nonempty() {
        let w = BraidWord::from_indices(&[1, -2]);
        let s = format!("{w}");
        assert!(s.contains('σ'));
    }

    #[test]
    fn test_iter() {
        let w = BraidWord::from_indices(&[1, 2, 3]);
        assert_eq!(w.iter().count(), 3);
    }

    #[test]
    fn test_as_slice() {
        let w = BraidWord::from_indices(&[1, 2]);
        assert_eq!(w.as_slice().len(), 2);
    }

    #[test]
    fn test_clone_equality() {
        let w = BraidWord::from_indices(&[1, -2]);
        assert_eq!(w.clone(), w);
    }
}
