mod vtime;
mod ctime;

pub use vtime::sample_vartime;
pub use ctime::sample;

/// Approximation of sqrt(1/(2*ln(2))) as defined
/// in https://eprint.iacr.org/2018/1234.pdf page 6
/// 
/// Approximated to 14 decimals (~2^-46).
const THETA_0: f64 = 0.84932180028802;