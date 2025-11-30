use std::f32::consts::PI;

/// Blackman-Harris窓関数を適用
pub fn apply_blackman_harris_window(samples: &mut [f32]) {
    let n = samples.len() as f32;
    let a0 = 0.35875;
    let a1 = 0.48829;
    let a2 = 0.14128;
    let a3 = 0.01168;

    for (i, v) in samples.iter_mut().enumerate() {
        let x = i as f32;
        let w = a0 - a1 * (2.0 * PI * x / n).cos() + a2 * (4.0 * PI * x / n).cos()
            - a3 * (6.0 * PI * x / n).cos();
        *v *= w;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blackman_harris_window() {
        let mut samples = vec![1.0; 100];
        apply_blackman_harris_window(&mut samples);

        // 窓関数は両端で小さく、中央で大きくなる
        assert!(samples[0] < samples[50]);
        assert!(samples[99] < samples[50]);
    }
}
