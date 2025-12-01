use crate::constants::{GUITAR_FREQUENCIES, GUITAR_TOLERANCE, MAX_FREQUENCY, MIN_FREQUENCY};

/// 検出されたピーク周波数から基音候補を探し、ギター音にマッチするものを返す
pub fn detect_guitar_fundamental(
    freq: f32,
    spectrum: &[f32],
    min_bin: usize,
    max_bin: usize,
    padded_size: usize,
    sample_rate: usize,
    noise_floor: f32,
    max_val: f32,
) -> Option<f32> {
    // 基音候補を探す: 検出周波数の1/2, 1/3, 1/4をチェックして最も低いギター音を採用
    let mut candidates: Vec<(f32, f32)> = Vec::new(); // (freq, power)

    // 検出された周波数自体も候補に
    candidates.push((freq, max_val));

    // 1/2, 1/3, 1/4（倍音→基音）をチェック
    for divisor in [2.0f32, 3.0, 4.0] {
        let sub_freq = freq / divisor;
        let sub_freq_bin = (sub_freq * padded_size as f32 / sample_rate as f32) as usize;
        if sub_freq_bin >= min_bin && sub_freq_bin < max_bin {
            let sub_idx = sub_freq_bin - min_bin;
            if sub_idx < spectrum.len() {
                let sub_val = spectrum[sub_idx];
                // ノイズフロアの1.5倍以上のピークがあれば候補に追加
                if sub_val > noise_floor * 1.5 {
                    candidates.push((sub_freq, sub_val));
                }
            }
        }
    }

    // 周波数の低い順にソート
    candidates.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // 候補の中からギター音にマッチするものを探す（低い周波数優先）
    for (candidate_freq, candidate_power) in &candidates {
        for &target in &GUITAR_FREQUENCIES {
            let ratio = candidate_freq / target;
            if ratio > (1.0 - GUITAR_TOLERANCE) && ratio < (1.0 + GUITAR_TOLERANCE) {
                // パワーが十分あるか確認（メインピークの10%以上）
                if *candidate_power > max_val * 0.1 {
                    return Some(*candidate_freq);
                }
            }
        }
    }

    None
}

/// 周波数がギター音の範囲内かどうかを判定
pub fn is_guitar_frequency(freq: f32) -> bool {
    GUITAR_FREQUENCIES.iter().any(|&target| {
        let ratio = freq / target;
        ratio > (1.0 - GUITAR_TOLERANCE) && ratio < (1.0 + GUITAR_TOLERANCE)
    })
}

/// ガウシアン補間でより正確な周波数を算出
pub fn gaussian_interpolation(
    spectrum: &[f32],
    peak_idx: usize,
    min_bin: usize,
    sample_rate: usize,
    padded_size: usize,
) -> f32 {
    let peak_bin = peak_idx + min_bin;

    if peak_idx > 0 && peak_idx < spectrum.len() - 1 {
        let y0 = (spectrum[peak_idx - 1] + 1e-10).ln();
        let y1 = (spectrum[peak_idx] + 1e-10).ln();
        let y2 = (spectrum[peak_idx + 1] + 1e-10).ln();

        // ガウシアン補間: delta = 0.5 * (y0 - y2) / (y0 - 2*y1 + y2)
        let denom = y0 - 2.0 * y1 + y2;
        let delta = if denom.abs() > 1e-10 {
            0.5 * (y0 - y2) / denom
        } else {
            0.0
        };

        // より精密な周波数計算（ゼロパディング後のサイズを使用）
        let precise_bin = peak_bin as f32 + delta.clamp(-0.5, 0.5);
        precise_bin * sample_rate as f32 / padded_size as f32
    } else {
        peak_bin as f32 * sample_rate as f32 / padded_size as f32
    }
}

/// スペクトルの中央値からノイズフロアを計算
pub fn calculate_noise_floor(spectrum: &[f32]) -> f32 {
    let mut sorted_spectrum = spectrum.to_vec();
    sorted_spectrum.sort_by(|a, b| a.partial_cmp(b).unwrap());
    sorted_spectrum[sorted_spectrum.len() / 2]
}

/// 周波数解析範囲のbin番号を計算
/// 戻り値: (min_bin, max_bin) - 常に min_bin < max_bin を保証
pub fn calculate_frequency_bins(sample_rate: usize, padded_size: usize) -> (usize, usize) {
    let min_bin = (MIN_FREQUENCY * padded_size as f32 / sample_rate as f32) as usize;
    let max_bin = std::cmp::min(
        (MAX_FREQUENCY * padded_size as f32 / sample_rate as f32) as usize,
        padded_size / 2,
    );
    // min_bin < max_bin を保証
    if max_bin <= min_bin {
        (min_bin, min_bin + 1)
    } else {
        (min_bin, max_bin)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_guitar_frequency() {
        // E2 (82.41Hz) の範囲内
        assert!(is_guitar_frequency(82.0));
        assert!(is_guitar_frequency(85.0));

        // 範囲外
        assert!(!is_guitar_frequency(50.0));
        assert!(!is_guitar_frequency(500.0));
    }

    #[test]
    fn test_calculate_noise_floor() {
        let spectrum = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(calculate_noise_floor(&spectrum), 3.0);
    }
}
