# discrete-gaussian [![Build](https://img.shields.io/circleci/build/github/chancehudson/discrete-gaussian/main)](https://dl.circleci.com/status-badge/redirect/gh/chancehudson/discrete-gaussian/tree/main)

Discrete gaussian samples implemented using [FFACT](https://eprint.iacr.org/2018/1234.pdf) constant time sampling.

## log

- Implemented vartime sampling for `u32`

TODO:

- Implement sampling for [`scalarff::FieldElement`](https://docs.rs/scalarff/latest/scalarff/trait.FieldElement.html)
- Implement sampling for [`ring_math::PolynomialRingElement`](https://github.com/chancehudson/ashlang/blob/main/ring-math/src/polynomial_ring.rs#L25)