#![doc = include_str!("../README.md")]
#![doc(html_root_url = "https://docs.rs/discrete-gaussian")]

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
