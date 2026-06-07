//! # braid-group-rs
//!
//! A research-grade braid group mathematics library implementing Artin generators,
//! braid composition, inverse computation, and word reduction.
//!
//! # Modules
//!
//! - [`generator`] — Artin generators σ_i and their inverses
//! - [`braid`] — Braid elements and operations
//! - [`word`] — Braid word representation and manipulation
//! - [`reduction`] — Word reduction algorithms
//! - [`group`] — Braid group B_n structure

pub mod braid;
pub mod generator;
pub mod group;
pub mod reduction;
pub mod word;

pub use braid::Braid;
pub use generator::Generator;
pub use group::BraidGroup;
