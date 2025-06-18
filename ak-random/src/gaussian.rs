// Beasley-Springer-Moro algorithm for approximating the inverse normal.
pub fn approx_inverse_gaussian(u: f64) -> f64 {
    // Constants for the approximation
    const A0: f64 = 2.50662823884;
    const A1: f64 = -18.61500062529;
    const A2: f64 = 41.39119773534;
    const A3: f64 = -25.44106049637;

    const B0: f64 = -8.47351093090;
    const B1: f64 = 23.08336743743;
    const B2: f64 = -21.06224101826;
    const B3: f64 = 3.13082909833;

    const C0: f64 = 0.3374754822726147;
    const C1: f64 = 0.9761690190917186;
    const C2: f64 = 0.1607979714918209;
    const C3: f64 = 0.0276438810333863;
    const C4: f64 = 0.0038405729373609;
    const C5: f64 = 0.0003951896511919;
    const C6: f64 = 0.0000321767881768;
    const C7: f64 = 0.0000002888167364;
    const C8: f64 = 0.0000003960315187;

    if u <= 0.0 || u >= 1.0 {
        panic!("Input must be in the range (0, 1)");
    }

    let y = u - 0.5;
    if y.abs() < 0.42 {
        let r = y * y;
        (A0 + y * (A1 + r * (A2 + r * A3))) / (1.0 + r * (B0 + r * (B1 + r * (B2 + r * B3))))
    } else {
        let mut r = u;
        if y > 0.0 {
            r = 1.0 - u;
        }
        r = (-1.0 * r.ln()).ln();
        let x = C0
            + r * (C1 + r * (C2 + r * (C3 + r * (C4 + r * (C5 + r * (C6 + r * (C7 + r * C8)))))));
        if y < 0.0 { -x } else { x }
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
            let result = approx_inverse_gaussian(*input);
            assert!(
                (result - expected).abs() < 1e-6,
                "Failed for input: {}",
                input
            );
        }
    }
}
