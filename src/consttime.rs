
use super::vartime;

fn gaussian_sample<R: rand::Rng>(k: u32, rng: &mut R) -> u32 {
    if k == 0 {
        panic!("k value must be greater than 0");
    }

    // range 0..=(k-1)
    let y = f64::from(rng.gen_range(0..k));
    // todo: gaussian sample
    let x = f64::from(vartime::sample_theta_0_vartime(rng));
    let t = y * (y + 2.0 * f64::from(k) * x);
    let a = -1.0 * t / f64::from(k * k);
    // z should end up being positive
    // because the floor of a negative value
    // is a larger absolute value
    let z = a - a.floor();
    let s = pow2_approx(z);
    println!("{a}");
    let f = s * 2_f64.powf(a.floor());
    println!("sf {} {}", s, f);
    // let f_mantissa = (f.to_bits() & ((1_u64 << 53) - 1) << 11) >> 11;
    // let f_exponent = (f.to_bits() & ((1_u64 << 11) - 1));
    // let f_mantissa = f.trunc();
    let f_mantissa = f.to_bits() & ((1 << 53) - 1);
    let f_exponent = i32::try_from((f.to_bits() >> 53) & (((1 << 11) - 1) )).unwrap();
    println!("mantissa: {f_mantissa} exponent: {} f: {f}", (2_f64).powi(f_exponent));
    // println!("{}", (1.0 + f_mantissa.powf(-1.0 * f64::from(f64::MANTISSA_DIGITS))));
    // println!("{} + {}", 1.0 + f_mantissa * 2_f64.powf(-1.0 * f64::from(f64::MANTISSA_DIGITS))) * (2_f64).powi(f_exponent))
    #[cfg(debug_assertions)]
    assert_eq!((1.0 + f64::from(u32::try_from(f_mantissa / 2_u64.pow(f64::MANTISSA_DIGITS)).unwrap()))* (2_f64).powi(f_exponent), f);


1
    // panic!();
}

/// Constant time approximation of 2^x where x is between 0 and 1 non-inclusive.
/// A polynomial approximation is calculated using the sollya tool `guessdegree`
/// and `fpminimax` as described in the [FACCT paper](https://eprint.iacr.org/2018/1234.pdf)
/// (page 8 and 9). The `supnorm` command is used to estimate the relative error of the
/// approximation as ~2^-45.
///
/// This approximation is conjectured to be constant time as it involves only
/// floating point addition and multiplication.
///
/// Input range checks are only applied in debug builds.
pub fn pow2_approx(x: f64) -> f64 {
    #[cfg(debug_assertions)]
    if x < 0.0 || x >= 1.0 {
        panic!("pow2_approx is only accurate over the interval (0, 1). Received {x}")
    }
    return 1.0 + x * (0.69314718056193380668617010087473317980766296386719
    + x * (0.24022650687652774559310842050763312727212905883789
    + x * (5.5504109841318247098307381293125217780470848083496e-2
    + x * (9.6181209331756452318717975913386908359825611114502e-3
    + x * (1.3333877552501097445841748978523355617653578519821e-3
    + x * (1.5396043210538638053991311593904356413986533880234e-4
    + x * (1.5359914219462011698283041005730353845137869939208e-5
    + x * (1.2303944375555413249736938854916878938183799618855e-6
    + x * 1.43291003789439094275872613876154915146798884961754e-7))))))));
}
