mod vtime;
mod ctime;

pub use vtime::sample_vartime;
pub use ctime::sample;

/// Approximation of sqrt(1/(2*ln(2))) as defined
/// in https://eprint.iacr.org/2018/1234.pdf page 6
/// 
/// Error due to IEEE754 conversion: -0.000000017699060302485625
/// Actual stored value: 0.849321782588958740234375
const THETA_0: f64 = 0.84932180028801904272;
