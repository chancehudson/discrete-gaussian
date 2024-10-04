mod vartime;
mod consttime;

/// Approximation of sqrt(1/(2*ln(2))) to 20 decimal places.
/// as defined in https://eprint.iacr.org/2018/1234.pdf page 6
const THETA_0: f64 = 0.84932180028801904272;
