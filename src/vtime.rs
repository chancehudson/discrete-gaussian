//! Variable time discrete sampling over algebraic structures.
//!

use rand::distributions::Bernoulli;
use rand::distributions::Distribution;

use crate::THETA_0;
use super::ctime::pow2_1024;

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
        let p_i = euler_50_approx(-1.0 * pow2_1024(f64::from(i) / (2.0 * theta * theta)));
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

/// 50 bit euler approximation in f64
/// returns e^x
/// 
/// variable time implementation due to division operator
pub fn euler_50_approx(x: f64) -> f64 {
    let p1: f64 = 1.66666666666666019037 * 10_f64.powf(-1_f64);
    let p2: f64 = -2.77777777770155933842 * 10_f64.powf(-3_f64);
    let p3: f64 = 6.61375632143793436117 * 10_f64.powf(-5_f64);
    let p4: f64 = -1.65339022054652515390_f64 * 10_f64.powf(-6_f64);
    let p5 = 4.13813679705723846039 * 10_f64.powf(-8_f64);
    let s = x / 2_f64;
    let t = s * s;
    // Let c = s−t·(p1 +t·(p2 +t·(p3 +t·(p4 +t·p5))))
    // explicitly write horner evaluation i guess
    let c = s - t * (p1 + t * (p2 + t * (p3 + t * (p4 + t * p5))));
    // Let r = 1 − ((s · c) / (c − 2) − s).
    let r = 1_f64 - ((s * c) / (c - 2_f64) - s);
    return r * r;
}

/// Losslessly extract the mantissa from an f64. Discard
/// the decimal portion, rounding toward 0.
pub fn f64_to_u64(x: f64) -> u64{
    x.to_bits() & ((1 << (64 - f64::MANTISSA_DIGITS)) - 1)
}

#[test]
fn generate_samples() {
    let target_theta = 30.0;
    let k: u32 = f64_to_u64(THETA_0 / target_theta).try_into().unwrap();
    for _ in 0..1000 {
        sample_vartime(k, &mut rand::thread_rng());
    }
}
