// Beasley-Springer-Moro algorithm for approximating the inverse normal.
use thiserror::Error;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum InverseGaussianError {
    #[error("input must be in the range (0, 1)")]
    OutOfRange,
}

pub type Result<T> = std::result::Result<T, InverseGaussianError>;

const A0: f64 = 2.506_628_238_84;
const A1: f64 = -18.615_000_625_29;
const A2: f64 = 41.391_197_735_34;
const A3: f64 = -25.441_060_496_37;

const B0: f64 = -8.473_510_930_90;
const B1: f64 = 23.083_367_437_43;
const B2: f64 = -21.062_241_018_26;
const B3: f64 = 3.130_829_098_33;

const C0: f64 = 0.337_475_482_272_614_7;
const C1: f64 = 0.976_169_019_091_718_6;
const C2: f64 = 0.160_797_971_491_820_9;
const C3: f64 = 0.027_643_881_033_386_3;
const C4: f64 = 0.003_840_572_937_360_9;
const C5: f64 = 0.000_395_189_651_191_9;
const C6: f64 = 0.000_032_176_788_176_8;
const C7: f64 = 0.000_000_288_816_736_4;
const C8: f64 = 0.000_000_396_031_518_7;

pub fn approx_inverse_gaussian(u: f64) -> Result<f64> {
    if !(0.0 < u && u < 1.0) {
        return Err(InverseGaussianError::OutOfRange);
    }

    let y = u - 0.5;
    if y.abs() < 0.42 {
        let r = y * y;
        Ok((A0 + y * (A1 + r * (A2 + r * A3)))
            / (1.0 + r * (B0 + r * (B1 + r * (B2 + r * B3)))))
    } else {
        let mut r = u;
        if y > 0.0 {
            r = 1.0 - u;
        }
        r = (-1.0 * r.ln()).ln();
        let x = C0
            + r * (C1 + r * (C2 + r * (C3 + r * (C4 + r * (C5 + r * (C6 + r * (C7 + r * C8)))))));
        Ok(if y < 0.0 { -x } else { x })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_approx_inverse_gaussian() {
        let inputs = [0.01, 0.99];
        let expected_outputs = [
            -2.3263478740408408, // Approximate for 0.01
            2.3263478740408408,  // Approximate for 0.99
        ];

        for (input, &expected) in inputs.iter().zip(expected_outputs.iter()) {
            let result = approx_inverse_gaussian(*input).expect("valid input");
            assert!(
                (result - expected).abs() < 1e-6,
                "Failed for input: {}",
                input
            );
        }
    }
}
