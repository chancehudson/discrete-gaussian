//! Variable time discrete sampling over algebraic structures.
//!

use rand::distributions::Bernoulli;
use rand::distributions::Distribution;

use crate::THETA_0;

/// Vartime u32 gaussian sampling
pub fn sample_vartime<R: rand::Rng>(k: u32, rng: &mut R) -> u32 {
    let bits = 32;
    let theta: f64 = f64::from(k) * THETA_0;
    let x = sample_theta_0_vartime(rng);
    let y = rng.gen_range(0..k);
    let z = k * x + y;
    let t = y * (y + 2 * k * x);
    let mut b = 1;
    for i in (0..bits).rev() {
        let p_i = f64::exp(-1.0 * f64::exp2(f64::from(i)) / (2.0 * theta * theta));
        let t_i = (t >> i) & 1;
        let d = Bernoulli::new(p_i).unwrap();
        let v = d.sample(&mut rand::thread_rng());
        b = b * (1 - t_i + u32::from(v) * t_i);
    }
    if b == 0 {
        return sample_vartime(k, rng);
    }
    z
}

/// Vartime probabilistic sampling from the theta_0 discrete
/// gaussian distribution evaluated over the positive integers.
///
/// theta_0 = sqrt(1/(2*ln(2))) as defined in https://eprint.iacr.org/2018/1234.pdf
/// page 6
pub fn sample_theta_0_vartime<R: rand::Rng>(rng: &mut R) -> u32 {
    let b = rng.gen_range(0..=1);
    if b == 0 {
        return 0;
    }
    let mut i = 1;
    loop {
        for _ in 0..(2 * i - 2) {
            if rng.gen_range(0..=1) != 0 {
                return sample_theta_0_vartime(rng);
            }
        }
        if rng.gen_range(0..=1) == 0 {
            return i;
        }
        i += 1;
    }
}

#[test]
fn generate_samples() {
    for _ in 0..1000 {
        println!("{}", sample_vartime(30, &mut rand::thread_rng()));
    }
}
