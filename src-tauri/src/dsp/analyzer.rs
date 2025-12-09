use rustfft::num_complex::Complex;
use rustfft::FftPlanner;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use tauri::image::Image;
use tauri::Emitter;

use crate::constants::{
    determine_color_with_hysteresis, TuningColor, CHANNEL_MODE, CUSTOM_PITCH, DROP_TUNING_ENABLED,
    DROP_TUNING_NOTE, FFT_SIZE, GUITAR_FREQUENCIES, LAST_TUNING_INFO, PITCH_MODE, RMS_THRESHOLD,
    STOP_FLAG, THRESHOLD_RATIO, TRAY_ICON_MODE, TRAY_ICON_STATE, TUNING_SHIFT,
};
use crate::dsp::frequency::{
    calculate_frequency_bins, calculate_noise_floor, detect_guitar_fundamental,
    gaussian_interpolation, is_guitar_frequency,
};
use crate::dsp::window::apply_blackman_harris_window;

/// 現在の設定に基づいて基準A4周波数を取得
fn get_effective_a4() -> f32 {
    match PITCH_MODE.load(Ordering::SeqCst) {
        0 => 440.0,                         // Standard
        1 => *CUSTOM_PITCH.read().unwrap(), // Custom
        2 => {
            // Shift: A4を半音単位でシフト
            let shift = TUNING_SHIFT.load(Ordering::SeqCst);
            440.0 * 2.0_f32.powf(shift as f32 / 12.0)
        }
        _ => 440.0,
    }
}

/// 6弦の目標周波数を取得（ドロップ考慮）
fn get_string6_target_freq() -> f32 {
    if DROP_TUNING_ENABLED.load(Ordering::SeqCst) {
        let note = DROP_TUNING_NOTE.load(Ordering::SeqCst);
        match note {
            0 => 73.42, // D2
            1 => 69.30, // C#2
            2 => 65.41, // C2
            3 => 61.74, // B1
            _ => 82.41, // E2 (default)
        }
    } else {
        82.41 // E2
    }
}

/// 周波数から音名とセント値を計算
fn calculate_note_info(freq: f32) -> (String, f32, f32) {
    const NOTE_NAMES: [&str; 12] = [
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    ];
    let a4_freq = get_effective_a4();

    // A4からの半音数を計算
    let semitones_from_a4 = 12.0 * (freq / a4_freq).log2();
    let nearest_semitone = semitones_from_a4.round() as i32;
    let cents = (semitones_from_a4 - nearest_semitone as f32) * 100.0;

    // 音名とオクターブを計算（A4 = MIDIノート69）
    let midi_note = 69 + nearest_semitone;
    let note_index = ((midi_note % 12) + 12) % 12;
    let octave = (midi_note / 12) - 1;
    let note_name = format!("{}{}", NOTE_NAMES[note_index as usize], octave);

    // 最も近いギター弦の周波数を探す
    // 6弦の周波数を動的に取得
    let string6_freq = get_string6_target_freq();
    // 基準ピッチのシフトを考慮したギター周波数リストを作成
    let shift_ratio = a4_freq / 440.0;

    let mut target_freqs = GUITAR_FREQUENCIES.to_vec();
    // 6弦を更新
    target_freqs[0] = string6_freq;

    // 全体をシフト
    let shifted_targets: Vec<f32> = target_freqs.iter().map(|&f| f * shift_ratio).collect();

    let target_freq = shifted_targets
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

/// 5x7ピクセルの大きめビットマップフォント（A-G, #）
fn get_char_bitmap_large(c: char) -> Option<[[bool; 5]; 7]> {
    match c {
        'A' => Some([
            [false, true, true, true, false],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, true, true, true, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
        ]),
        'B' => Some([
            [true, true, true, true, false],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, true, true, true, false],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, true, true, true, false],
        ]),
        'C' => Some([
            [false, true, true, true, false],
            [true, false, false, false, true],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, true],
            [false, true, true, true, false],
        ]),
        'D' => Some([
            [true, true, true, true, false],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, true, true, true, false],
        ]),
        'E' => Some([
            [true, true, true, true, true],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, true, true, true, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, true, true, true, true],
        ]),
        'F' => Some([
            [true, true, true, true, true],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, true, true, true, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
        ]),
        'G' => Some([
            [false, true, true, true, false],
            [true, false, false, false, true],
            [true, false, false, false, false],
            [true, false, true, true, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [false, true, true, true, false],
        ]),
        '#' => Some([
            [false, true, false, true, false],
            [false, true, false, true, false],
            [true, true, true, true, true],
            [false, true, false, true, false],
            [true, true, true, true, true],
            [false, true, false, true, false],
            [false, true, false, true, false],
        ]),
        _ => None,
    }
}

/// 文字列を大きめビットマップとして描画（スケーリング対応）
fn draw_text_large(
    rgba: &mut [u8],
    size: usize,
    text: &str,
    start_x: usize,
    start_y: usize,
    color: [u8; 4],
    scale: usize,
) {
    let mut x_offset = start_x;
    for c in text.chars() {
        if let Some(bitmap) = get_char_bitmap_large(c) {
            for (row, bits) in bitmap.iter().enumerate() {
                for (col, &pixel) in bits.iter().enumerate() {
                    if pixel {
                        // スケーリング: 各ピクセルをscale x scaleで描画
                        for sy in 0..scale {
                            for sx in 0..scale {
                                let x = x_offset + col * scale + sx;
                                let y = start_y + row * scale + sy;
                                if x < size && y < size {
                                    let idx = (y * size + x) * 4;
                                    rgba[idx..idx + 4].copy_from_slice(&color);
                                }
                            }
                        }
                    }
                }
            }
        }
        x_offset += 6 * scale; // 文字幅5 + 間隔1
    }
}

/// セント値に応じてチューニング状態を示すアイコンを生成（32x32 RGBA）
/// モード0: インジケーターのみ（全画面メーター）
/// モード1: 音名を大きく上部に、チューニングバーを下部に表示
fn generate_tuning_icon(cents: f32, color: TuningColor, note_name: &str) -> Vec<u8> {
    const SIZE: usize = 32;
    let mut rgba = vec![0u8; SIZE * SIZE * 4];

    // 表示モードを取得（0=インジケーターのみ, 1=インジケーター+音名）
    let icon_mode = TRAY_ICON_MODE.load(Ordering::SeqCst);

    // 背景色（濃いグレー）
    let bg_color: [u8; 4] = [40, 40, 40, 255];

    // メーターの色を決定（ヒステリシス付きの色を使用）
    let meter_color: [u8; 4] = match color {
        TuningColor::Green => [0, 255, 100, 255], // 緑: チューニング良好
        TuningColor::Yellow => [255, 200, 0, 255], // 黄: 少しずれ
        TuningColor::Red => [255, 60, 60, 255],   // 赤: 大きくずれ
    };

    // 中央線の色
    let center_line_color: [u8; 4] = [80, 80, 80, 255];
    // テキストの色（白）
    let text_color: [u8; 4] = [255, 255, 255, 255];

    // 背景を塗りつぶし
    for y in 0..SIZE {
        for x in 0..SIZE {
            let idx = (y * SIZE + x) * 4;
            rgba[idx..idx + 4].copy_from_slice(&bg_color);
        }
    }

    // メーター領域の設定（モードに応じて変更）
    let meter_top;
    let meter_bottom = SIZE - 2;
    let meter_left = 2;
    let meter_right = SIZE - 2; // 30 (2〜29の28px幅)
    let meter_width = meter_right - meter_left; // 28px

    if icon_mode == 1 {
        // モード1: 音名+インジケーター
        // 音名を上部に大きく描画（音名+#のみ抽出：例 "A#4" -> "A#"）
        let note_chars: String = note_name
            .chars()
            .filter(|c| c.is_alphabetic() || *c == '#')
            .collect();

        // 大きなフォントで描画（5x7ピクセル、スケール2 = 10x14ピクセル）
        let scale = 2;
        let char_count = note_chars.len();
        // 文字幅: 5*scale + 間隔1*scale = 6*scale per char、最後の間隔除く
        let text_width = if char_count > 0 {
            char_count * 6 * scale - scale
        } else {
            0
        };
        let text_start_x = (SIZE.saturating_sub(text_width)) / 2;
        draw_text_large(
            &mut rgba,
            SIZE,
            &note_chars,
            text_start_x,
            1,
            text_color,
            scale,
        );

        // メーター領域（音名の下に配置）- 音名が14px高さなので、17pxから開始
        meter_top = 17;
    } else {
        // モード0: インジケーターのみ（全画面メーター）
        meter_top = 2;
    }

    // 偶数幅なので中央は2ピクセル（15と16）
    let center_x1 = meter_left + meter_width / 2 - 1; // 15
    let center_x2 = meter_left + meter_width / 2; // 16

    // セント値をメーター位置に変換（-10〜+10セント → 左端〜右端）
    // ±10セントを超えると端にクロップ
    let clamped_cents = cents.clamp(-10.0, 10.0);
    let normalized = (clamped_cents + 10.0) / 20.0; // 0.0〜1.0
                                                    // インジケーターは2px幅なので、位置は左側のピクセルを基準にする
    let indicator_pos = meter_left as f32 + normalized * (meter_width - 2) as f32;
    let indicator_x1 = indicator_pos.round() as usize;
    let indicator_x2 = indicator_x1 + 1;

    // メーターバーを描画
    for y in meter_top..meter_bottom {
        for x in meter_left..meter_right {
            let idx = (y * SIZE + x) * 4;

            // 中央線を描画（2ピクセル幅）
            if x == center_x1 || x == center_x2 {
                rgba[idx..idx + 4].copy_from_slice(&center_line_color);
            }

            // インジケーターを描画（常に2ピクセル幅）
            if x == indicator_x1 || x == indicator_x2 {
                rgba[idx..idx + 4].copy_from_slice(&meter_color);
            }
        }
    }

    // 中央マーク（小さな三角形）を上部に描画
    let triangle_color: [u8; 4] = [120, 120, 120, 255];
    for i in 0..2 {
        let y = meter_top.saturating_sub(1 + i);
        if y >= 1 && (icon_mode == 1 && y > 15 || icon_mode == 0) {
            for dx in 0..=i {
                // 左側の中央点から左へ
                let x1 = center_x1.saturating_sub(dx);
                if x1 < SIZE {
                    let idx = (y * SIZE + x1) * 4;
                    rgba[idx..idx + 4].copy_from_slice(&triangle_color);
                }
                // 右側の中央点から右へ
                let x2 = center_x2 + dx;
                if x2 < SIZE {
                    let idx = (y * SIZE + x2) * 4;
                    rgba[idx..idx + 4].copy_from_slice(&triangle_color);
                }
            }
        }
    }

    rgba
}

/// トレイアイコンを動的に更新（デバウンス・ヒステリシス付き）
fn update_tray_icon(app_handle: &tauri::AppHandle, cents: f32, note_name: &str) {
    // インジケーター位置を計算（-50〜+50の整数）
    let indicator_pos = cents.clamp(-50.0, 50.0).round() as i32;

    // 状態管理を取得
    let mut state = match TRAY_ICON_STATE.lock() {
        Ok(s) => s,
        Err(_) => return,
    };

    // ヒステリシス付きで色を決定
    let new_color = determine_color_with_hysteresis(cents, state.last_color);

    // 更新が必要か判定
    if !state.should_update(new_color, indicator_pos) {
        return;
    }

    // トレイアイコンを更新
    let tray = app_handle.tray_by_id("main");
    if let Some(tray) = tray {
        let icon_data = generate_tuning_icon(cents, new_color, note_name);
        let image = Image::new_owned(icon_data, 32, 32);
        let _ = tray.set_icon(Some(image));

        // 状態を更新
        state.update(new_color, indicator_pos);
    }
}

/// トレイアイコンを即座に再描画（モード変更時など、デバウンスをスキップ）
pub fn refresh_tray_icon(app_handle: &tauri::AppHandle, cents: f32, note_name: &str) {
    // 状態管理を取得
    let state = match TRAY_ICON_STATE.lock() {
        Ok(s) => s,
        Err(_) => return,
    };

    // 現在の色を取得（なければ新規計算）
    let color = state
        .last_color
        .unwrap_or_else(|| determine_color_with_hysteresis(cents, None));
    drop(state); // ロックを解放

    // トレイアイコンを更新
    let tray = app_handle.tray_by_id("main");
    if let Some(tray) = tray {
        let icon_data = generate_tuning_icon(cents, color, note_name);
        let image = Image::new_owned(icon_data, 32, 32);
        let _ = tray.set_icon(Some(image));
    }
}

/// トレイアイコンのツールチップを更新
fn update_tray_tooltip(app_handle: &tauri::AppHandle, note_name: &str, freq: f32, cents: f32) {
    let tray = app_handle.tray_by_id("main");
    if let Some(tray) = tray {
        // セント値を整数に丸める
        let cents_int = cents.round() as i32;
        // +/-の表示（0は符号なし）
        let direction = if cents_int > 0 {
            "+"
        } else if cents_int < 0 {
            "" // 負の数は自動で-がつく
        } else {
            "" // 0の場合は符号なし
        };
        let tooltip = format!(
            "Guitar Tuner\n{} ({:.1}Hz)\n{}{}¢",
            note_name, freq, direction, cents_int
        );
        let _ = tray.set_tooltip(Some(&tooltip));
    }
}

/// トレイアイコンを初期状態にリセット
fn reset_tray_icon(app_handle: &tauri::AppHandle) {
    use tauri::image::Image;

    // 状態をリセット
    if let Ok(mut state) = TRAY_ICON_STATE.lock() {
        state.reset();
    }

    let tray = app_handle.tray_by_id("main");
    if let Some(tray) = tray {
        // アイコンを初期状態に戻す
        if let Ok(icon) = Image::from_path("icons/icon.ico") {
            let _ = tray.set_icon(Some(icon));
        } else if let Ok(icon) = Image::from_path("icons/32x32.png") {
            let _ = tray.set_icon(Some(icon));
        }
        // ツールチップを初期状態に戻す
        let _ = tray.set_tooltip(Some("Guitar Tuner"));
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

    // リセット判定用の定数（秒）
    const RESET_TIMEOUT_SECS: u64 = 3;

    thread::spawn(move || {
        let mut planner = FftPlanner::<f32>::new();

        // 周波数安定化用のバッファ
        let mut freq_history: Vec<f32> = Vec::with_capacity(10);

        // 最後に有効な音を検出した時刻
        let mut last_valid_sound_time: Option<Instant> = None;
        // リセット済みフラグ（連続してresetイベントを送信しないため）
        let mut is_reset = true;

        loop {
            // 停止フラグがセットされたらスレッドを終了
            if STOP_FLAG.load(Ordering::SeqCst) {
                println!("解析スレッド終了");
                break;
            }
            thread::sleep(Duration::from_millis(50));

            // リセット判定: 有効な音が一定時間検出されなかったらリセット
            if let Some(last_time) = last_valid_sound_time {
                if last_time.elapsed() > Duration::from_secs(RESET_TIMEOUT_SECS) && !is_reset {
                    // リセットイベントを送信
                    let _ = app_handle.emit("reset", ());
                    // トレイアイコンをリセット
                    reset_tray_icon(&app_handle);
                    // 履歴をクリア
                    freq_history.clear();
                    is_reset = true;
                    println!(
                        "リセットイベント送信: {}秒間音が検出されませんでした",
                        RESET_TIMEOUT_SECS
                    );
                }
            }

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

                                // 有効な音を検出したので時刻を更新
                                last_valid_sound_time = Some(Instant::now());
                                is_reset = false;

                                // チューニング情報を計算してトレイのツールチップを更新
                                let (note_name, _target_freq, cents) =
                                    calculate_note_info(median_freq);

                                // グローバル変数を更新
                                if let Ok(mut info) = LAST_TUNING_INFO.lock() {
                                    info.note_name = note_name.clone();
                                    info.frequency = median_freq;
                                    info.cents = cents;
                                }

                                // トレイのツールチップとアイコンを更新
                                update_tray_tooltip(&app_handle, &note_name, median_freq, cents);
                                update_tray_icon(&app_handle, cents, &note_name);
                            }
                        }
                    }
                }
            }
        }
    });
}
