//! [![Build](https://img.shields.io/circleci/build/github/chancehudson/discrete-gaussian/main)](https://dl.circleci.com/status-badge/redirect/gh/chancehudson/discrete-gaussian/tree/main) [![Docs](https://img.shields.io/docsrs/discrete-gaussian)](https://docs.rs/discrete-gaussian) [![Version](https://img.shields.io/crates/v/discrete-gaussian)](https://crates.io/crates/discrete-gaussian)
//!
//! Discrete gaussian samples based on [FACCT](https://eprint.iacr.org/2018/1234.pdf) and [DDLL13](https://eprint.iacr.org/2013/383.pdf).
//! Implemented with ~40 bits of accuracy (relative error <= 2^-40).
//! Based on the analysis in [WALT19](https://eprint.iacr.org/2019/068.pdf) this implies an upper bound of 2^80 security in constructions.
//!

pub mod ctime;
pub mod vtime;

pub use ctime::sample;
pub use vtime::sample_vartime;
pub use vtime::sample_vartime_k;

/// Approximation of `sqrt(1/(2*ln(2)))` as defined
/// in [FACCT](https://eprint.iacr.org/2018/1234.pdf) page 6
///
/// Approximated to 14 decimals (~2^-46).
pub const THETA_0: f64 = 0.84932180028802;

/// Approximate a `k` value for the requested theta.
pub fn k_from_theta(theta: f64) -> u32 {
    // using `as` here should be okay as long as
    // theta / THETA_0 < 2^32
    // most standard deviations (theta) should be < 10000
    ((theta / THETA_0).trunc() as u64).try_into().unwrap()
}
