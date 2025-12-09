use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;
use tauri::command;

use crate::audio::{find_device_by_name, get_input_device_names, start_audio_stream};
use crate::constants::{
    CHANNEL_MODE, CUSTOM_PITCH, DROP_TUNING_ENABLED, DROP_TUNING_NOTE, LAST_TUNING_INFO,
    PITCH_MODE, STOP_FLAG, STREAM_ID, THRESHOLD_RATIO, TRAY_ICON_MODE, TUNING_SHIFT,
};
use crate::dsp::{refresh_tray_icon, run_analysis_thread};

/// チャンネルモードを設定（0=左, 1=右, 2=両方の平均）
#[command]
pub fn set_channel_mode(mode: u32) -> Result<(), String> {
    CHANNEL_MODE.store(mode.min(2), Ordering::SeqCst);
    println!("Channel mode set to: {}", mode);
    Ok(())
}

/// 現在のチャンネルモードを取得
#[command]
pub fn get_channel_mode() -> u32 {
    CHANNEL_MODE.load(Ordering::SeqCst)
}

/// 基準ピッチモードを設定 (0=standard, 1=custom, 2=shift)
#[command]
#[allow(dead_code)]
pub fn set_pitch_mode(mode: u32) {
    PITCH_MODE.store(mode, Ordering::SeqCst);
    println!("Pitch mode set to: {}", mode);
}

/// カスタム基準ピッチを設定 (438.0-445.0 Hz)
#[command]
#[allow(dead_code)]
pub fn set_custom_pitch(pitch: f32) -> Result<(), String> {
    if !(438.0..=445.0).contains(&pitch) {
        return Err("Pitch must be between 438 and 445 Hz".to_string());
    }
    *CUSTOM_PITCH.write().unwrap() = pitch;
    println!("Custom pitch set to: {:.1} Hz", pitch);
    Ok(())
}

/// チューニングシフトを設定（半音数、負の値）
#[command]
#[allow(dead_code)]
pub fn set_tuning_shift(semitones: i32) {
    TUNING_SHIFT.store(semitones, Ordering::SeqCst);
    println!("Tuning shift set to: {} semitones", semitones);
}

/// 6弦ドロップチューニングを設定
#[command]
#[allow(dead_code)]
pub fn set_drop_tuning(enabled: bool, note: u32) {
    DROP_TUNING_ENABLED.store(enabled, Ordering::SeqCst);
    DROP_TUNING_NOTE.store(note, Ordering::SeqCst);
    println!(
        "Drop tuning set to: enabled={}, note={}",
        enabled,
        match note {
            0 => "D",
            1 => "C#",
            2 => "C",
            3 => "B",
            _ => "Unknown",
        }
    );
}

/// トレイアイコン表示モードを設定（0=インジケーターのみ, 1=インジケーター+音名）
#[command]
pub fn set_tray_icon_mode(app: tauri::AppHandle, mode: u32) -> Result<(), String> {
    TRAY_ICON_MODE.store(mode.min(1), Ordering::SeqCst);
    println!("Tray icon mode set to: {}", mode);

    // 最後のチューニング情報を取得して即座にアイコンを再描画
    if let Ok(info) = LAST_TUNING_INFO.lock() {
        if !info.note_name.is_empty() {
            refresh_tray_icon(&app, info.cents, &info.note_name);
        }
    }

    Ok(())
}

/// 現在のトレイアイコン表示モードを取得
#[command]
pub fn get_tray_icon_mode() -> u32 {
    TRAY_ICON_MODE.load(Ordering::SeqCst)
}

/// 閾値を設定（1.1〜10.0の範囲）
#[command]
pub fn set_threshold(ratio: f32) -> Result<(), String> {
    let mut threshold = THRESHOLD_RATIO.lock().map_err(|e| e.to_string())?;
    *threshold = ratio.max(1.1).min(10.0);
    println!("Threshold set to: {:.2}", *threshold);
    Ok(())
}

/// 現在の閾値を取得
#[command]
pub fn get_threshold() -> Result<f32, String> {
    let threshold = THRESHOLD_RATIO.lock().map_err(|e| e.to_string())?;
    Ok(*threshold)
}

/// オーディオデバイスリストを取得
#[command]
pub fn get_audio_devices() -> Result<Vec<String>, String> {
    get_input_device_names()
}

/// 指定されたデバイスでオーディオ入力の監視を開始
#[command]
pub fn start_listening(app: tauri::AppHandle, device_name: String) -> Result<(), String> {
    // 既存の解析スレッドを停止
    STOP_FLAG.store(true, Ordering::SeqCst);
    thread::sleep(Duration::from_millis(150)); // スレッド終了を待つ
    STOP_FLAG.store(false, Ordering::SeqCst);

    // 新しいストリームIDを発行（古いスレッドを停止させる）
    let current_stream_id = STREAM_ID.fetch_add(1, Ordering::SeqCst) + 1;
    println!("Starting stream ID: {}", current_stream_id);

    // デバイスを検索
    let device = find_device_by_name(&device_name)?;

    // オーディオストリームを開始
    let (buffer, sample_rate, channels) = start_audio_stream(&device)?;

    // 解析スレッドを起動
    run_analysis_thread(app, buffer, sample_rate, channels);

    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub device_name: Option<String>,
    pub threshold: Option<f32>,
    pub channel_mode: Option<u32>,
    pub tray_icon_mode: Option<u32>,
    // 新規追加
    pub pitch_mode: Option<String>, // "standard" | "custom" | "shift"
    pub custom_pitch: Option<f32>,  // 438.0-445.0
    pub tuning_shift: Option<i32>,  // -1〜-5
    pub drop_tuning_enabled: Option<bool>, // ドロップチューニング有効/無効
    pub drop_tuning_note: Option<String>, // "D" | "C#" | "C" | "B"
    pub theme_mode: Option<String>, // "system" | "light" | "dark"
}

fn settings_path() -> PathBuf {
    // exeと同じディレクトリ
    std::env::current_exe()
        .map(|mut p| {
            p.pop();
            p.push("settings.json");
            p
        })
        .unwrap_or_else(|_| PathBuf::from("settings.json"))
}

#[command]
pub fn get_settings() -> Result<Settings, String> {
    let path = settings_path();
    if !path.exists() {
        return Ok(Settings {
            device_name: None,
            threshold: None,
            channel_mode: None,
            tray_icon_mode: None,
            pitch_mode: None,
            custom_pitch: None,
            tuning_shift: None,
            drop_tuning_enabled: None,
            drop_tuning_note: None,
            theme_mode: None,
        });
    }
    let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    serde_json::from_str(&content).map_err(|e| e.to_string())
}

#[command]
pub fn set_settings(settings: Settings) -> Result<(), String> {
    let path = settings_path();
    let json = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())
}
