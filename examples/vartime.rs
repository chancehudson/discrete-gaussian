use std::time::Instant;

use discrete_gaussian::k_from_theta;
use discrete_gaussian::sample_vartime_k;
use discrete_gaussian::THETA_0;

pub fn main() {
    let sample_count = 1000;
    let target_theta = 27000.0;
    println!("Sampling with target theta: {target_theta}");
    let k: u32 = k_from_theta(target_theta);
    println!("Approximated k = {k}");
    println!("Approximated theta: {}", THETA_0 * f64::from(k));
    let now = Instant::now();
    for _ in 0..sample_count {
        sample_vartime_k(k, &mut rand::thread_rng());
    }
    let elapsed = now.elapsed();
    println!("{sample_count} samples in: {:.2?}", elapsed);
}
