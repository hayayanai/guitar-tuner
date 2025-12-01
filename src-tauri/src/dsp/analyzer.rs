use rustfft::num_complex::Complex;
use rustfft::FftPlanner;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tauri::image::Image;
use tauri::Emitter;

use crate::constants::{CHANNEL_MODE, FFT_SIZE, GUITAR_FREQUENCIES, LAST_TUNING_INFO, RMS_THRESHOLD, STOP_FLAG, THRESHOLD_RATIO};
use crate::dsp::frequency::{
    calculate_frequency_bins, calculate_noise_floor, detect_guitar_fundamental,
    gaussian_interpolation, is_guitar_frequency,
};
use crate::dsp::window::apply_blackman_harris_window;

/// 周波数から音名とセント値を計算
fn calculate_note_info(freq: f32) -> (String, f32, f32) {
    const NOTE_NAMES: [&str; 12] = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
    const A4_FREQ: f32 = 440.0;

    // A4からの半音数を計算
    let semitones_from_a4 = 12.0 * (freq / A4_FREQ).log2();
    let nearest_semitone = semitones_from_a4.round() as i32;
    let cents = (semitones_from_a4 - nearest_semitone as f32) * 100.0;

    // 音名とオクターブを計算（A4 = MIDIノート69）
    let midi_note = 69 + nearest_semitone;
    let note_index = ((midi_note % 12) + 12) % 12;
    let octave = (midi_note / 12) - 1;
    let note_name = format!("{}{}", NOTE_NAMES[note_index as usize], octave);

    // 最も近いギター弦の周波数を探す
    let target_freq = GUITAR_FREQUENCIES
        .iter()
        .min_by(|a, b| {
            let diff_a = (freq - *a).abs();
            let diff_b = (freq - *b).abs();
            diff_a.partial_cmp(&diff_b).unwrap()
        })
        .copied()
        .unwrap_or(freq);

    (note_name, target_freq, cents)
}

/// セント値に応じてチューニング状態を示すアイコンを生成（32x32 RGBA）
/// タスクマネージャーのCPUグラフのように、メーター形式で状態を表示
fn generate_tuning_icon(cents: f32) -> Vec<u8> {
    const SIZE: usize = 32;
    let mut rgba = vec![0u8; SIZE * SIZE * 4];

    // 背景色（濃いグレー）
    let bg_color: [u8; 4] = [40, 40, 40, 255];

    // メーターの色を決定
    let abs_cents = cents.abs();
    let meter_color: [u8; 4] = if abs_cents < 5.0 {
        [0, 255, 100, 255]   // 緑: チューニング良好
    } else if abs_cents < 15.0 {
        [255, 200, 0, 255]   // 黄: 少しずれ
    } else {
        [255, 60, 60, 255]   // 赤: 大きくずれ
    };

    // 中央線の色
    let center_line_color: [u8; 4] = [100, 100, 100, 255];

    // 背景を塗りつぶし
    for y in 0..SIZE {
        for x in 0..SIZE {
            let idx = (y * SIZE + x) * 4;
            rgba[idx..idx + 4].copy_from_slice(&bg_color);
        }
    }

    // メーター領域（上下に余白を残す）
    let meter_top = 6;
    let meter_bottom = SIZE - 6;
    let meter_left = 2;
    let meter_right = SIZE - 2;
    let meter_width = meter_right - meter_left;

    // 中央位置
    let center_x = meter_left + meter_width / 2;

    // セント値をメーター位置に変換（-50〜+50セント → 左端〜右端）
    let clamped_cents = cents.clamp(-50.0, 50.0);
    let normalized = (clamped_cents + 50.0) / 100.0; // 0.0〜1.0
    let indicator_x = meter_left + (normalized * meter_width as f32) as usize;

    // インジケーターの幅（セント値の絶対値が小さいほど細く、正確さを強調）
    let indicator_width = if abs_cents < 5.0 { 4 } else if abs_cents < 15.0 { 5 } else { 6 };
    let half_width = indicator_width / 2;

    // メーターバーを描画
    for y in meter_top..meter_bottom {
        for x in meter_left..meter_right {
            let idx = (y * SIZE + x) * 4;

            // 中央線を描画
            if x == center_x || x == center_x + 1 {
                rgba[idx..idx + 4].copy_from_slice(&center_line_color);
            }

            // インジケーターを描画
            if x >= indicator_x.saturating_sub(half_width) && x <= indicator_x + half_width {
                rgba[idx..idx + 4].copy_from_slice(&meter_color);
            }
        }
    }

    // 中央マーク（三角形）を上部に描画
    let triangle_color: [u8; 4] = [180, 180, 180, 255];
    for i in 0..4 {
        let y = meter_top - 1 - i;
        if y > 0 {
            for dx in 0..=i {
                let x1 = center_x.saturating_sub(dx);
                let x2 = center_x + dx + 1;
                if x1 < SIZE {
                    let idx = (y * SIZE + x1) * 4;
                    rgba[idx..idx + 4].copy_from_slice(&triangle_color);
                }
                if x2 < SIZE {
                    let idx = (y * SIZE + x2) * 4;
                    rgba[idx..idx + 4].copy_from_slice(&triangle_color);
                }
            }
        }
    }

    rgba
}

/// トレイアイコンを動的に更新
fn update_tray_icon(app_handle: &tauri::AppHandle, cents: f32) {
    let tray = app_handle.tray_by_id("main");
    if let Some(tray) = tray {
        let icon_data = generate_tuning_icon(cents);
        let image = Image::new_owned(icon_data, 32, 32);
        let _ = tray.set_icon(Some(image));
    }
}

/// トレイアイコンのツールチップを更新
fn update_tray_tooltip(app_handle: &tauri::AppHandle, note_name: &str, freq: f32, cents: f32) {
    let tray = app_handle.tray_by_id("main");
    if let Some(tray) = tray {
        let direction = if cents > 0.0 { "+" } else { "" };
        let tooltip = format!(
            "Guitar Tuner\n{} ({:.1}Hz)\n{}{}¢",
            note_name, freq, direction, cents as i32
        );
        let _ = tray.set_tooltip(Some(&tooltip));
    }
}

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

                                // チューニング情報を計算してトレイのツールチップを更新
                                let (note_name, _target_freq, cents) = calculate_note_info(median_freq);

                                // グローバル変数を更新
                                if let Ok(mut info) = LAST_TUNING_INFO.lock() {
                                    info.note_name = note_name.clone();
                                    info.frequency = median_freq;
                                    info.cents = cents;
                                }

                                // トレイのツールチップとアイコンを更新
                                update_tray_tooltip(&app_handle, &note_name, median_freq, cents);
                                update_tray_icon(&app_handle, cents);
                            }
                        }
                    }
                }
            }
        }
    });
}
