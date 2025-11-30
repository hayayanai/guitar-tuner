use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Stream;
use tauri::Emitter;
use tauri::command;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::thread;
use std::time::Duration;
use rustfft::{FftPlanner, num_complex::Complex};
use once_cell::sync::Lazy;

// ストリームをグローバルに保持（dropされないようにする）
static STREAM: Lazy<Mutex<Option<Stream>>> = Lazy::new(|| Mutex::new(None));

// 閾値設定（グローバル）
static THRESHOLD_RATIO: Lazy<Mutex<f32>> = Lazy::new(|| Mutex::new(2.0));

// チャンネル選択（0=左, 1=右, 2=両方の平均）
static CHANNEL_MODE: AtomicU32 = AtomicU32::new(1); // デフォルトは右チャンネル

// 解析スレッド制御用
static STREAM_ID: AtomicU32 = AtomicU32::new(0);
static STOP_FLAG: AtomicBool = AtomicBool::new(false);

#[command]
fn set_channel_mode(mode: u32) -> Result<(), String> {
  CHANNEL_MODE.store(mode.min(2), Ordering::SeqCst);
  println!("Channel mode set to: {}", mode);
  Ok(())
}

#[command]
fn get_channel_mode() -> u32 {
  CHANNEL_MODE.load(Ordering::SeqCst)
}

#[command]
fn set_threshold(ratio: f32) -> Result<(), String> {
  let mut threshold = THRESHOLD_RATIO.lock().map_err(|e| e.to_string())?;
  *threshold = ratio.max(1.1).min(10.0); // 1.1〜10.0の範囲に制限
  println!("Threshold set to: {:.2}", *threshold);
  Ok(())
}

#[command]
fn get_threshold() -> Result<f32, String> {
  let threshold = THRESHOLD_RATIO.lock().map_err(|e| e.to_string())?;
  Ok(*threshold)
}

#[command]
fn start_listening(app: tauri::AppHandle, device_name: String) -> Result<(), String> {
  // 既存の解析スレッドを停止
  STOP_FLAG.store(true, Ordering::SeqCst);
  thread::sleep(Duration::from_millis(150)); // スレッド終了を待つ
  STOP_FLAG.store(false, Ordering::SeqCst);

  // 新しいストリームIDを発行（古いスレッドを停止させる）
  let current_stream_id = STREAM_ID.fetch_add(1, Ordering::SeqCst) + 1;
  println!("Starting stream ID: {}", current_stream_id);

  let host = cpal::default_host();
  let device = host.input_devices()
    .map_err(|e| e.to_string())?
    .find(|d| d.name().map(|n| n == device_name).unwrap_or(false))
    .ok_or_else(|| format!("デバイスが見つかりません: {}", device_name))?;

  let config = device.default_input_config().map_err(|e| e.to_string())?;
  let sample_rate = config.sample_rate().0 as usize;
  let channels = config.channels() as usize;
  let fft_size = 16384; // 高精度のため16384に増加（分解能: 約2.9Hz @48kHz）
  let buffer_size = fft_size * channels;
  let buffer = Arc::new(Mutex::new(Vec::<f32>::with_capacity(buffer_size)));
  let buffer_clone = buffer.clone();

  println!("Audio config: sample_rate={}, channels={}, fft_size={}", sample_rate, channels, fft_size);

  let app_handle = app.clone();
  let err_fn = |err| eprintln!("Stream error: {}", err);

  // F32でストリーム作成
  let buffer_clone_f32 = buffer_clone.clone();
  let stream = device.build_input_stream(
    &config.clone().into(),
    move |data: &[f32], _| {
      let mut buf = buffer_clone_f32.lock().unwrap();
      buf.extend_from_slice(data);
      if buf.len() > buffer_size {
        let len = buf.len();
        buf.drain(..len - buffer_size);
      }
    },
    err_fn,
    None
  ).map_err(|e| e.to_string())?;

  stream.play().map_err(|e| format!("ストリーム開始失敗: {}", e))?;

  // ストリームをグローバルに保持（dropされないようにする）
  {
    let mut global_stream = STREAM.lock().unwrap();
    *global_stream = Some(stream);
  }

  // 解析スレッド起動
  let app_handle_clone = app_handle.clone();
  let buffer_for_analysis = buffer.clone(); // 解析スレッド用にバッファをクローン
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
        let buf = buffer_for_analysis.lock().unwrap();
        if buf.len() < buffer_size {
          continue;
        }
        samples = buf.clone();
      }

      // チャンネルモードを取得
      let channel_mode = CHANNEL_MODE.load(Ordering::SeqCst);

      // モノラル化（チャンネルモードに応じて選択）
      let mono: Vec<f32> = if channels == 2 {
        samples.chunks(2)
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

      if mono.len() < fft_size {
        continue;
      }

      // 信号レベルチェック
      let rms: f32 = (mono.iter().take(fft_size).map(|x| x * x).sum::<f32>() / fft_size as f32).sqrt();

      // RMSレベルをフロントエンドに送信（dBスケールで-80〜0dB → 0〜1）
      let db = 20.0 * rms.max(0.0000001).log10();
      let level = ((db + 80.0) / 80.0).clamp(0.0, 1.0); // -80dB〜0dB を 0〜1 に正規化
      let _ = app_handle_clone.emit("input_level", level);

      // RMS閾値（ノイズフロア以下はスキップ）
      if rms < 0.001 {
        continue;
      }

      // Blackman-Harris窓を適用してからゼロパディング
      let mut windowed: Vec<f32> = mono.iter().take(fft_size).cloned().collect();
      let fft_size_f = fft_size as f32;
      for (i, v) in windowed.iter_mut().enumerate() {
        let n = i as f32;
        let a0 = 0.35875;
        let a1 = 0.48829;
        let a2 = 0.14128;
        let a3 = 0.01168;
        let w = a0
          - a1 * (2.0 * std::f32::consts::PI * n / fft_size_f).cos()
          + a2 * (4.0 * std::f32::consts::PI * n / fft_size_f).cos()
          - a3 * (6.0 * std::f32::consts::PI * n / fft_size_f).cos();
        *v *= w;
      }

      // 2倍のゼロパディング（周波数分解能を2倍に向上）
      let padded_size = fft_size * 2;
      let mut input: Vec<Complex<f32>> = vec![Complex{ re: 0.0, im: 0.0 }; padded_size];
      for (i, &v) in windowed.iter().enumerate() {
        input[i] = Complex{ re: v, im: 0.0 };
      }

      let fft_padded = planner.plan_fft_forward(padded_size);
      fft_padded.process(&mut input);

      // パワースペクトル（E2〜E4+αの範囲 75-350Hz に限定）
      // ゼロパディングしたサイズでbin計算
      let min_bin = (75.0 * padded_size as f32 / sample_rate as f32) as usize;
      let max_bin = std::cmp::min((350.0 * padded_size as f32 / sample_rate as f32) as usize, padded_size / 2);
      if max_bin <= min_bin {
        continue;
      }
      let spectrum: Vec<f32> = input[min_bin..max_bin].iter().map(|c| c.norm()).collect();

      // ノイズフロアを計算（スペクトルの中央値）
      let mut sorted_spectrum = spectrum.clone();
      sorted_spectrum.sort_by(|a, b| a.partial_cmp(b).unwrap());
      let noise_floor = sorted_spectrum[sorted_spectrum.len() / 2];

      // ピーク検出
      if let Some((max_idx, max_val)) = spectrum.iter().enumerate().max_by(|a, b| a.1.partial_cmp(b.1).unwrap()) {
        // ガウシアン補間でより正確な周波数を求める
        let peak_bin = max_idx + min_bin;
        let freq = if max_idx > 0 && max_idx < spectrum.len() - 1 {
          let y0 = (spectrum[max_idx - 1] + 1e-10).ln();
          let y1 = (spectrum[max_idx] + 1e-10).ln();
          let y2 = (spectrum[max_idx + 1] + 1e-10).ln();

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
        };

        // 現在の閾値を取得
        let threshold_ratio = *THRESHOLD_RATIO.lock().unwrap();

        // ピークがノイズフロアの閾値倍以上なら有効な信号
        if *max_val > noise_floor * threshold_ratio {
          // 生の周波数をリアルタイム送信（閾値を超えた場合のみ）
          let _ = app_handle_clone.emit("raw_frequency", freq);

          // レギュラーチューニングの各弦の周波数（低い順）
          // E2: 82.41Hz, A2: 110Hz, D3: 146.83Hz, G3: 196Hz, B3: 246.94Hz, E4: 329.63Hz
          let guitar_freqs = [82.41f32, 110.0, 146.83, 196.0, 246.94, 329.63];
          let tolerance = 0.15; // ±15% の範囲

          // 基音候補を探す: 検出周波数の1/2, 1/3, 1/4をチェックして最も低いギター音を採用
          let mut candidates: Vec<(f32, f32)> = Vec::new(); // (freq, power)

          // 検出された周波数自体も候補に
          candidates.push((freq, *max_val));

          // 1/2（2倍音→基音）をチェック
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

          // 候補の中からギター音にマッチするものを探す（低い周波数優先）
          let mut adjusted_freq = freq;
          let mut found_match = false;

          // 周波数の低い順にソート
          candidates.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

          for (candidate_freq, candidate_power) in &candidates {
            // ギター音にマッチするかチェック
            for &target in &guitar_freqs {
              let ratio = candidate_freq / target;
              if ratio > (1.0 - tolerance) && ratio < (1.0 + tolerance) {
                // パワーが十分あるか確認（メインピークの10%以上）
                if *candidate_power > *max_val * 0.1 {
                  adjusted_freq = *candidate_freq;
                  found_match = true;
                  break;
                }
              }
            }
            if found_match {
              break;
            }
          }

          // ギター音にマッチしなければ元の周波数を使用
          let is_guitar_note = guitar_freqs.iter().any(|&target| {
            let ratio = adjusted_freq / target;
            ratio > (1.0 - tolerance) && ratio < (1.0 + tolerance)
          });

          if is_guitar_note {
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

              // 常に送信（微細な変化も反映）
              let _ = app_handle_clone.emit("frequency", median_freq);
            }
          }
        }
      }
    }
  });
  Ok(())
}

#[command]
fn get_audio_devices() -> Result<Vec<String>, String> {
  let host = cpal::default_host();
  let devices = host.input_devices().map_err(|e| e.to_string())?;
  let mut names = Vec::new();
  for device in devices {
    if let Ok(name) = device.name() {
      names.push(name);
    }
  }
  Ok(names)
}

pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_audio_devices, start_listening, set_threshold, get_threshold, set_channel_mode, get_channel_mode])
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
