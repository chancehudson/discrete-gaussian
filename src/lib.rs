mod vtime;
mod ctime;

pub use vtime::sample_vartime;
pub use vtime::sample_vartime_k;
pub use ctime::sample;

/// Approximation of sqrt(1/(2*ln(2))) as defined
/// in https://eprint.iacr.org/2018/1234.pdf page 6
/// 
/// Approximated to 14 decimals (~2^-46).
pub const THETA_0: f64 = 0.84932180028802;

/// Approximate a `k` value for the requested theta.
pub fn k_from_theta(theta: f64) -> u32 {
    // using `as` here should be okay 
    ((theta / THETA_0).trunc() as u64).try_into().unwrap()
}

/// Losslessly extract the mantissa from an f64. Discard
/// the decimal portion, rounding toward 0.
pub fn f64_to_u64(x: f64) -> u64{
    x as u64
}