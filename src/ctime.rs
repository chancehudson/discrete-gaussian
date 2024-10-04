/// Functions that are designed to be constant time.
/// 
use super::vtime;

/// 53 bit accurate representation of 2^x
/// for x in 0..1. Similar to the exponentiation
/// function described in [FACCT](https://eprint.iacr.org/2018/1234.pdf).
///
/// This approximation is conjectured to be constant time as it involves only
/// floating point addition and multiplication.
/// 
/// ~27 f64 multiplications and additions
///
/// Input range checks are only applied in debug builds.
/// 
/// sollya
/// > guessdegree(1, [0, 1], 1b-53, 1/2^x);
/// [11;11]
/// > fpminimax(2^x, 11, [|1, D...|],[0,1], floating, relative);
pub fn pow2_unit(x: f64) -> f64 {
    #[cfg(debug_assertions)]
    if x < 0.0 || x > 1.0 {
        panic!("discrete_gaussian::pow2_unit is only accurate over the interval [0, 1]. Received {x}")
    }
    return 1.0 + x * (0.69314718055994595236057875808910466730594635009766 + x * (0.24022650695906239137755733281665015965700149536133 + x * (5.5504108665615274620375174663422512821853160858154e-2 + x * (9.618129099454324551499162510026508243754506111145e-3 + x * (1.33335586337906793555352358282561908708885312080383e-3 + x * (1.54035121930721417794415972757349209132371470332146e-4 + x * (1.52531771389177337269358811222552674280450446531177e-5 + x * (1.32083432055888705908372905001302299865528766531497e-6 + x * (1.02533862337995870705910481015393775905408801918384e-7 + x * (6.5588174947098374579243424450491345423230882261123e-9 + x * 6.264729520369589538179779228995857492945376066018e-10))))))))));
}

/// 53 bit accurate approximation of 2^x
/// for x in 0..14. Outputs between 1 and 16384
/// are valid.
/// 
/// This approximation is conjectured to be constant time as it involves only
/// floating point addition and multiplication.
/// 
/// ~27 f64 multiplications and additions
/// 
/// Input range checks are only applied in debug builds.
/// 
/// sollya
/// > guessdegree(1, [0, 10], 1b-53, 1/2^x);
/// [21;23]
/// > fpminimax(2^x, 23, [|1, D...|],[0,10], floating, relative);
pub fn pow2_1024(x: f64) -> f64 {
    #[cfg(debug_assertions)]
    if x < 0.0 || x > 10.0 {
        panic!("discrete_gaussian::pow2_1024 is only accurate over the interval [0, 10]. Received {x}")
    }
    return 1.0 + x * (0.69314718055994550827136890802648849785327911376953 + x * (0.240226506959095309490237468708073720335960388183594 + x * (5.5504108664874436673830615518454578705132007598877e-2 + x * (9.618129107362902857625286401344055775552988052368e-3 + x * (1.3333558154467158377021185344801779137924313545227e-3 + x * (1.54035302326176556280204477111794858501525595784187e-4 + x * (1.5252736059632826076367176360015776026557432487607e-5 + x * (1.32154636433563851675206807234630446146184112876654e-6 + x * (1.01782650488605204924307619154116855142433450964745e-7 + x * (7.0538444469489690078874994251089541874577548696834e-9 + x * (4.4505204301213882429468699346579614595231788598539e-10 + x * (2.5494110711616183275263032447986949555890356933219e-11 + x * (1.42362104575936261046082409822022030480501153526518e-12 + x * (5.4870791832499226537989656621114453947094836971932e-14 + x * (5.5928791694160161597531718135293885527814126620849e-15 + x * (-2.3994436552596069537103031787881535201032345751976e-16 + x * (5.1206979070956915002194844659936350512351050523344e-17 + x * (-4.1584717724561422608141392385244177377702796469922e-18 + x * (3.3093653888837137356760928910791661506849987846483e-19 + x * (-1.76248323004841628919215317851191624541796092282214e-20 + x * (7.1368233850692392262984763188905597797427107545382e-22 + x * (-1.7630486394378449660389333891054260997251802657839e-23 + x * 2.39759652843898776507354327086164592546857185532465e-25))))))))))))))))))))));
}

pub fn sample<R: rand::Rng>(k: u32, rng: &mut R) -> u32 {
    if k == 0 {
        panic!("k value must be greater than 0");
    }

    // range 0..=(k-1)
    let y = f64::from(rng.gen_range(0..k));
    // todo: gaussian sample
    let x = f64::from(vtime::sample_theta_0_vartime(rng));
    let t = y * (y + 2.0 * f64::from(k) * x);
    let a = -1.0 * t / f64::from(k * k);
    // z should end up being positive
    // because the floor of a negative value
    // is a larger absolute value
    let z = a - a.floor();
    let s = pow2_unit(z);
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

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;

    #[test]
    fn pow2_unit_test() {
        assert_eq!(pow2_unit(0.0), 1.0);
        assert_eq!(pow2_unit(1.0), 2.0);

        let mut rng = rand::thread_rng();
        let iterations = 1000;
        let interval: f64 = rng.gen();

        let mut exponent = 0.0;
        let max_difference = 10e-16;
        for _ in 0..iterations {
            let expected = f64::exp2(exponent);
            let actual = pow2_unit(exponent);
            if (expected - actual).abs() > max_difference {
                panic!("difference exceeded acceptable amount");
            }
            exponent += interval;
            println!("pow2_unit({exponent})");
            if exponent > 1.0 {
                exponent -= 1.0;
            }
        }
    }

    #[test]
    fn pow2_1024_test() {
        assert_eq!(pow2_1024(0.0), 1.0);
        // assert_eq!(pow2_16384(14.0), 16384.0);

        let mut rng = rand::thread_rng();
        let iterations = 1000;
        let interval: f64 = {
            let mut out = 0.0;
            for _ in 0..10 {
                let r: f64 = rng.gen();
                out += r;
            }
            out
        };

        let mut exponent = 0.0;
        let max_difference = 10e-13;
        for _ in 0..iterations {
            let expected = f64::exp2(exponent);
            let actual = pow2_1024(exponent);
            if (expected - actual).abs() > max_difference {
                println!("expected: {expected} actual: {actual} difference: {}", (expected - actual).abs());
                panic!("difference exceeded acceptable amount");
            }
            exponent += interval;
            println!("pow2_1024({exponent})");
            if exponent > 10.0 {
                exponent -= 10.0;
            }
        }
    }
}