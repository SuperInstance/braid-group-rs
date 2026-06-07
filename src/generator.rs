//! Artin generators for braid groups.
//!
//! In the braid group B_n, the Artin generators are σ_1, σ_2, ..., σ_{n-1}.
//! Each σ_i represents crossing strand i over strand i+1.
//! The inverse σ_i^{-1} (written as -i) represents crossing strand i under strand i+1.

/// An Artin generator or its inverse in a braid group.
///
/// A positive index `i` represents σ_i, and a negative index `-i` represents σ_i^{-1}.
///
/// # Examples
///
/// ```
/// use braid_group_rs::generator::Generator;
///
/// let sigma1 = Generator::new(1);   // σ₁
/// let sigma1_inv = Generator::new(-1); // σ₁⁻¹
/// assert!(sigma1.is_positive());
/// assert!(sigma1_inv.is_negative());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Generator {
    /// The index: positive for σ_i, negative for σ_i^{-1}.
    index: i32,
}

impl Generator {
    /// Create a new generator with the given index.
    ///
    /// Positive indices represent σ_i, negative represent σ_i^{-1}.
    /// Zero is invalid and will panic.
    ///
    /// # Panics
    ///
    /// Panics if `index` is 0.
    pub fn new(index: i32) -> Self {
        assert_ne!(index, 0, "Generator index cannot be 0");
        Self { index }
    }

    /// Create σ_i (positive generator).
    pub fn sigma(i: i32) -> Self {
        assert!(i > 0, "sigma index must be positive");
        Self { index: i }
    }

    /// Create σ_i^{-1} (inverse generator).
    pub fn sigma_inv(i: i32) -> Self {
        assert!(i > 0, "sigma index must be positive");
        Self { index: -i }
    }

    /// Get the absolute value of the index (which crossing this generator refers to).
    pub fn abs_index(&self) -> i32 {
        self.index.abs()
    }

    /// Check if this is a positive generator (σ_i).
    pub fn is_positive(&self) -> bool {
        self.index > 0
    }

    /// Check if this is an inverse generator (σ_i^{-1}).
    pub fn is_negative(&self) -> bool {
        self.index < 0
    }

    /// Get the inverse of this generator.
    ///
    /// σ_i ↔ σ_i^{-1}
    pub fn inverse(&self) -> Generator {
        Generator { index: -self.index }
    }

    /// Check if this generator commutes with another.
    ///
    /// Two generators commute iff |i - j| > 1.
    pub fn commutes_with(&self, other: &Generator) -> bool {
        (self.abs_index() - other.abs_index()).abs() > 1
    }

    /// Check if two generators satisfy the braid relation: σ_i σ_j σ_i = σ_j σ_i σ_j
    /// when |i - j| = 1.
    pub fn braid_relation_pair(&self, other: &Generator) -> bool {
        (self.abs_index() - other.abs_index()).abs() == 1
    }

    /// Get the raw index value.
    pub fn index(&self) -> i32 {
        self.index
    }
}

impl std::fmt::Display for Generator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.index > 0 {
            write!(f, "σ{}", self.index)
        } else {
            write!(f, "σ{}⁻¹", -self.index)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_generator() {
        let g = Generator::new(3);
        assert_eq!(g.abs_index(), 3);
        assert!(g.is_positive());
        assert!(!g.is_negative());
    }

    #[test]
    fn test_negative_generator() {
        let g = Generator::new(-2);
        assert_eq!(g.abs_index(), 2);
        assert!(!g.is_positive());
        assert!(g.is_negative());
    }

    #[test]
    fn test_sigma_constructor() {
        let g = Generator::sigma(3);
        assert_eq!(g.index(), 3);
    }

    #[test]
    fn test_sigma_inv_constructor() {
        let g = Generator::sigma_inv(3);
        assert_eq!(g.index(), -3);
    }

    #[test]
    #[should_panic(expected = "cannot be 0")]
    fn test_zero_index_panics() {
        Generator::new(0);
    }

    #[test]
    fn test_inverse() {
        let g = Generator::new(3);
        let inv = g.inverse();
        assert_eq!(inv.index(), -3);
        assert_eq!(inv.inverse().index(), 3);
    }

    #[test]
    fn test_double_inverse_is_identity() {
        let g = Generator::new(-5);
        assert_eq!(g.inverse().inverse(), g);
    }

    #[test]
    fn test_commutes_far_generators() {
        let g1 = Generator::new(1);
        let g2 = Generator::new(3);
        assert!(g1.commutes_with(&g2));
    }

    #[test]
    fn test_does_not_commute_adjacent() {
        let g1 = Generator::new(1);
        let g2 = Generator::new(2);
        assert!(!g1.commutes_with(&g2));
    }

    #[test]
    fn test_does_not_commute_same() {
        let g1 = Generator::new(1);
        let g2 = Generator::new(1);
        assert!(!g1.commutes_with(&g2));
    }

    #[test]
    fn test_braid_relation_pair() {
        let g1 = Generator::new(1);
        let g2 = Generator::new(2);
        assert!(g1.braid_relation_pair(&g2));
        assert!(!g1.braid_relation_pair(&Generator::new(3)));
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Generator::new(1)), "σ1");
        assert_eq!(format!("{}", Generator::new(-2)), "σ2⁻¹");
    }

    #[test]
    fn test_equality() {
        assert_eq!(Generator::new(1), Generator::new(1));
        assert_ne!(Generator::new(1), Generator::new(-1));
    }
}
