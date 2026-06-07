//! Word reduction algorithms for braid words.
//!
//! Implements free reduction (canceling σ_i σ_i^{-1} pairs) and
//! braid relation application (σ_i σ_{i+1} σ_i ↔ σ_{i+1} σ_i σ_{i+1}).

use crate::generator::Generator;
use crate::word::BraidWord;

/// Perform free reduction on a braid word.
///
/// Cancels adjacent pairs of the form σ_i σ_i^{-1} or σ_i^{-1} σ_i.
/// This is a standard stack-based reduction.
///
/// # Examples
///
/// ```
/// use braid_group_rs::word::BraidWord;
/// use braid_group_rs::reduction::free_reduce;
///
/// let word = BraidWord::from_indices(&[1, 2, -2, -1]);
/// let reduced = free_reduce(&word);
/// assert!(reduced.is_empty());
/// ```
pub fn free_reduce(word: &BraidWord) -> BraidWord {
    let mut stack: Vec<Generator> = Vec::new();
    for g in word.iter() {
        if let Some(top) = stack.last() {
            if top.index() == -g.index() {
                stack.pop();
                continue;
            }
        }
        stack.push(*g);
    }
    BraidWord::from_vec(stack)
}

/// Apply braid relations: σ_i σ_{i+1} σ_i → σ_{i+1} σ_i σ_{i+1}.
///
/// This performs one pass over the word, applying the relation wherever possible.
/// Multiple passes may be needed for full simplification.
pub fn apply_braid_relations(word: &BraidWord) -> BraidWord {
    let gens = word.as_slice();
    if gens.len() < 3 {
        return word.clone();
    }

    let mut result: Vec<Generator> = Vec::new();
    let mut i = 0;

    while i + 2 < gens.len() {
        let a = gens[i];
        let b = gens[i + 1];
        let c = gens[i + 2];

        // Check for σ_i σ_{i±1} σ_i pattern (braid relation)
        if a.abs_index() == c.abs_index()
            && (a.abs_index() - b.abs_index()).unsigned_abs() == 1
            && a.index() == c.index()
            && a.is_positive() == b.is_positive()
        {
            // Replace a b a with b a b
            result.push(b);
            result.push(a);
            result.push(b);
            i += 3;
        } else {
            result.push(a);
            i += 1;
        }
    }

    // Add remaining generators
    while i < gens.len() {
        result.push(gens[i]);
        i += 1;
    }

    BraidWord::from_vec(result)
}

/// Compute the length of the fully reduced braid word.
///
/// This performs free reduction and returns the length of the result.
pub fn reduced_length(word: &BraidWord) -> usize {
    free_reduce(word).len()
}

/// Check if a braid word is freely reduced (no cancelable pairs).
pub fn is_freely_reduced(word: &BraidWord) -> bool {
    let gens = word.as_slice();
    for w in gens.windows(2) {
        if w[0].index() == -w[1].index() {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_free_reduce_empty() {
        let w = BraidWord::empty();
        assert!(free_reduce(&w).is_empty());
    }

    #[test]
    fn test_free_reduce_no_change() {
        let w = BraidWord::from_indices(&[1, 2, 3]);
        assert_eq!(free_reduce(&w).to_indices(), vec![1, 2, 3]);
    }

    #[test]
    fn test_free_reduce_simple_cancel() {
        let w = BraidWord::from_indices(&[1, -1]);
        assert!(free_reduce(&w).is_empty());
    }

    #[test]
    fn test_free_reduce_inverse_order() {
        let w = BraidWord::from_indices(&[-1, 1]);
        assert!(free_reduce(&w).is_empty());
    }

    #[test]
    fn test_free_reduce_partial() {
        let w = BraidWord::from_indices(&[1, 2, -2, 3]);
        assert_eq!(free_reduce(&w).to_indices(), vec![1, 3]);
    }

    #[test]
    fn test_free_reduce_nested() {
        let w = BraidWord::from_indices(&[1, 2, -1, -2]);
        // No adjacent canceling pairs: 1,2,-1,-2 -> 2,-2 -> empty? No.
        // Actually: 1,2,-1 don't cancel. Then -2. Stack: [1,2,-1,-2] -> no cancellation
        let result = free_reduce(&w);
        // 1 and 2 are not inverses, 2 and -1 not inverses, -1 and -2 not inverses
        assert_eq!(result.to_indices(), vec![1, 2, -1, -2]);
    }

    #[test]
    fn test_is_freely_reduced() {
        let w1 = BraidWord::from_indices(&[1, 2, 3]);
        assert!(is_freely_reduced(&w1));

        let w2 = BraidWord::from_indices(&[1, -1, 2]);
        assert!(!is_freely_reduced(&w2));
    }

    #[test]
    fn test_reduced_length() {
        let w = BraidWord::from_indices(&[1, -1, 2]);
        assert_eq!(reduced_length(&w), 1);
    }

    #[test]
    fn test_braid_relation_short_word() {
        let w = BraidWord::from_indices(&[1, 2]);
        let result = apply_braid_relations(&w);
        assert_eq!(result.to_indices(), vec![1, 2]);
    }

    #[test]
    fn test_braid_relation_identity() {
        let w = BraidWord::from_indices(&[1, 3, 1]); // |1-3| = 2, no relation
        let result = apply_braid_relations(&w);
        assert_eq!(result.to_indices(), vec![1, 3, 1]);
    }
}
