# braid-group-rs

Research-grade braid group mathematics library in pure Rust.

## Features

- **Artin generators**: σ_i with standard braid relations
- **Braid composition**: Concatenation of braid words
- **Inverse**: Computation of braid inverses
- **Word reduction**: Free reduction and basic simplification
- **Braid group B_n**: Support for any number of strands

## Usage

```rust
use braid_group_rs::braid::Braid;
use braid_group_rs::generator::Generator;

let b = Braid::from_generators(4, &[Generator::new(1), Generator::new(-2), Generator::new(1)]);
let inv = b.inverse();
let product = b.compose(&inv);
assert!(product.is_reduced_identity());
```

## License

MIT OR Apache-2.0
