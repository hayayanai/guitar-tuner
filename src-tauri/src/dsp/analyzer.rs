use rustfft::num_complex::Complex;
use rustfft::FftPlanner;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::Emitter;

use crate::constants::{CHANNEL_MODE, FFT_SIZE, RMS_THRESHOLD, STOP_FLAG, THRESHOLD_RATIO};
use crate::dsp::frequency::{
    calculate_frequency_bins, calculate_noise_floor, detect_guitar_fundamental,
    gaussian_interpolation, is_guitar_frequency,
};
use crate::dsp::window::apply_blackman_harris_window;

/// 周波数解析スレッドを起動
pub fn run_analysis_thread(
    app_handle: tauri::AppHandle,
    buffer: Arc<Mutex<Vec<f32>>>,
    sample_rate: usize,
    channels: usize,
) {
    let buffer_size = FFT_SIZE * channels;

    thread::spawn(move || {
        let mut planner = FftPlanner::<f32>::new();

        // 周波数安定化用のバッファ
        let mut freq_history: Vec<f32> = Vec::with_capacity(10);

        loop {
            // 停止フラグがセットされたらスレッドを終了
            if STOP_FLAG.load(Ordering::SeqCst) {
                println!("解析スレッド終了");
                break;
            }
            thread::sleep(Duration::from_millis(50));

            let samples: Vec<f32>;
            {
                let buf = buffer.lock().unwrap();
                if buf.len() < buffer_size {
                    continue;
                }
                samples = buf.clone();
            }

            // チャンネルモードを取得
            let channel_mode = CHANNEL_MODE.load(Ordering::SeqCst);

            // モノラル化（チャンネルモードに応じて選択）
            let mono: Vec<f32> = if channels == 2 {
                samples
                    .chunks(2)
                    .map(|chunk| {
                        if chunk.len() == 2 {
                            match channel_mode {
                                0 => chunk[0],                    // 左チャンネル
                                1 => chunk[1],                    // 右チャンネル
                                _ => (chunk[0] + chunk[1]) / 2.0, // 両方の平均
                            }
                        } else {
                            chunk[0]
                        }
                    })
                    .collect()
            } else {
                samples.clone()
            };

            if mono.len() < FFT_SIZE {
                continue;
            }

            // 信号レベルチェック
            let rms: f32 =
                (mono.iter().take(FFT_SIZE).map(|x| x * x).sum::<f32>() / FFT_SIZE as f32).sqrt();

            // RMSレベルをフロントエンドに送信（dBスケールで-80〜0dB → 0〜1）
            let db = 20.0 * rms.max(0.0000001).log10();
            let level = ((db + 80.0) / 80.0).clamp(0.0, 1.0);
            let _ = app_handle.emit("input_level", level);

            // RMS閾値（ノイズフロア以下はスキップ）
            if rms < RMS_THRESHOLD {
                continue;
            }

            // Blackman-Harris窓を適用
            let mut windowed: Vec<f32> = mono.iter().take(FFT_SIZE).cloned().collect();
            apply_blackman_harris_window(&mut windowed);

            // 2倍のゼロパディング（周波数分解能を2倍に向上）
            let padded_size = FFT_SIZE * 2;
            let mut input: Vec<Complex<f32>> = vec![Complex { re: 0.0, im: 0.0 }; padded_size];
            for (i, &v) in windowed.iter().enumerate() {
                input[i] = Complex { re: v, im: 0.0 };
            }

            let fft_padded = planner.plan_fft_forward(padded_size);
            fft_padded.process(&mut input);

            // パワースペクトル（E2〜E4+αの範囲に限定）
            let (min_bin, max_bin) = calculate_frequency_bins(sample_rate, padded_size);
            if max_bin <= min_bin {
                continue;
            }
            let spectrum: Vec<f32> = input[min_bin..max_bin].iter().map(|c| c.norm()).collect();

            // ノイズフロアを計算
            let noise_floor = calculate_noise_floor(&spectrum);

            // ピーク検出
            if let Some((max_idx, max_val)) = spectrum
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            {
                // ガウシアン補間でより正確な周波数を求める
                let freq =
                    gaussian_interpolation(&spectrum, max_idx, min_bin, sample_rate, padded_size);

                // 現在の閾値を取得
                let threshold_ratio = *THRESHOLD_RATIO.lock().unwrap();

                // ピークがノイズフロアの閾値倍以上なら有効な信号
                if *max_val > noise_floor * threshold_ratio {
                    // 生の周波数をリアルタイム送信（閾値を超えた場合のみ）
                    let _ = app_handle.emit("raw_frequency", freq);

                    // 基音検出を試みる
                    if let Some(adjusted_freq) = detect_guitar_fundamental(
                        freq,
                        &spectrum,
                        min_bin,
                        max_bin,
                        padded_size,
                        sample_rate,
                        noise_floor,
                        *max_val,
                    ) {
                        if is_guitar_frequency(adjusted_freq) {
                            // 履歴に追加
                            freq_history.push(adjusted_freq);
                            if freq_history.len() > 5 {
                                freq_history.remove(0);
                            }

                            // 2サンプル以上あれば中央値を計算して送信
                            if freq_history.len() >= 2 {
                                let mut sorted = freq_history.clone();
                                sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
                                let median_freq = sorted[sorted.len() / 2];

                                let _ = app_handle.emit("frequency", median_freq);
                            }
                        }
                    }
                }
            }
        }
    });
}
