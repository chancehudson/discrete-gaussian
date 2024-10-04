# discrete-gaussian [![Build](https://img.shields.io/circleci/build/github/chancehudson/discrete-gaussian/main)](https://dl.circleci.com/status-badge/redirect/gh/chancehudson/discrete-gaussian/tree/main)

Discrete gaussian samples.

## Example

`cargo run --example vartime --release`

```
Sampling with target theta: 20
Approximated k = 23
Approximated theta: 19.534401406624458
1000 samples in: 10.07ms
```

## Feature Tracking

- [x] variable time sampling in `u32`
  - [ ] in [`scalarff::FieldElement`](https://docs.rs/scalarff/latest/scalarff/trait.FieldElement.html)
  - [ ] in [`ring_math::PolynomialRingElement`](https://github.com/chancehudson/ashlang/blob/main/ring-math/src/polynomial_ring.rs#L25)
- [ ] arbitrary precision decimal math
- [ ] sample a larger 
- [ ]  cumulative distribution table macro
- [ ] constant time sampling in `u32` 
  - [ ] in [`scalarff::FieldElement`](https://docs.rs/scalarff/latest/scalarff/trait.FieldElement.html)
  - [ ] in [`ring_math::PolynomialRingElement`](https://github.com/chancehudson/ashlang/blob/main/ring-math/src/polynomial_ring.rs#L25)
- structure compatibility
  - [`ring_math::Matrix2D`](https://github.com/chancehudson/ashlang/blob/main/ring-math/src/matrix2d.rs#L7) compat, sample into a matrix
  - [`ring_math::Vector](https://github.com/chancehudson/ashlang/blob/main/ring-math/src/vector.rs#L6) compat, sample into a vector
