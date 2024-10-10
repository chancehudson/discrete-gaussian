# discrete-gaussian [![Build](https://img.shields.io/circleci/build/github/chancehudson/discrete-gaussian/main)](https://dl.circleci.com/status-badge/redirect/gh/chancehudson/discrete-gaussian/tree/main) [![Docs](https://img.shields.io/docsrs/discrete-gaussian)](https://docs.rs/discrete-gaussian) [![Version](https://img.shields.io/crates/v/discrete-gaussian)](https://crates.io/crates/discrete-gaussian)

Discrete gaussian samples based on [FACCT](https://eprint.iacr.org/2018/1234.pdf) and [DDLL13](https://eprint.iacr.org/2013/383.pdf). Implemented with ~40 bits of accuracy (relative error <= 2^-40). Based on the analysis in [WALT19](https://eprint.iacr.org/2019/068.pdf) this implies an upper bound of 2^80 security in constructions.

## Example

`cargo run --example vartime --release`

```
Sampling with target theta: 20
Approximated k = 23
Approximated theta: 19.534401406624458
1000 samples in: 16.90ms
```

## Feature Tracking

- [x] variable time sampling in `u32`
  - [ ] in [`discrete-gaussian::FieldElement`](https://docs.rs/scalarff/latest/scalarff/trait.FieldElement.html)
  - [ ] in [`ring_math::PolynomialRingElement`](https://github.com/chancehudson/ashlang/blob/main/ring-math/src/polynomial_ring.rs#L25)
- [ ] higher precision math
- [ ] cumulative distribution table
- [ ] constant time sampling in `u32`
  - [ ] in [`scalarff::FieldElement`](https://docs.rs/scalarff/latest/scalarff/trait.FieldElement.html)
  - [ ] in [`ring_math::PolynomialRingElement`](https://github.com/chancehudson/ashlang/blob/main/ring-math/src/polynomial_ring.rs#L25)
- structure compatibility
  - [`ring_math::Matrix2D`](https://github.com/chancehudson/ashlang/blob/main/ring-math/src/matrix2d.rs#L7) compat, sample into a matrix
  - [`ring_math::Vector`](https://github.com/chancehudson/ashlang/blob/main/ring-math/src/vector.rs#L6) compat, sample into a vector
