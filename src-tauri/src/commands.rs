use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;
use tauri::command;
use tauri::Manager;

use crate::audio::{find_device_by_name, get_input_device_names, start_audio_stream};
use crate::constants::{
    CHANNEL_MODE, CUSTOM_PITCH, DROP_TUNING_ENABLED, DROP_TUNING_NOTE, LAST_TUNING_INFO, LOCALE,
    PITCH_MODE, STOP_FLAG, STREAM_ID, THRESHOLD_RATIO, TRAY_ICON_MODE, TUNING_SHIFT,
};
use crate::dsp::{refresh_tray_icon, run_analysis_thread};

/// Supported locales
const SUPPORTED_LOCALES: [&str; 2] = ["en", "ja"];

/// Get localized tray menu text
pub fn get_tray_menu_text(locale: &str) -> (&'static str, &'static str) {
    match locale {
        "ja" => ("ウィンドウを表示", "終了"),
        _ => ("Show Window", "Quit"),
    }
}

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

/// Set whether the window should always be on top
#[command]
pub fn set_always_on_top(app: tauri::AppHandle, enabled: bool) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window
            .set_always_on_top(enabled)
            .map_err(|e| e.to_string())?;
        println!("Always on top set to: {}", enabled);
    }
    Ok(())
}

/// Set locale (en/ja)
#[command]
pub fn set_locale(app: tauri::AppHandle, locale: String) -> Result<(), String> {
    if !SUPPORTED_LOCALES.contains(&locale.as_str()) {
        return Err(format!(
            "Invalid locale. Must be one of: {}",
            SUPPORTED_LOCALES.join(", ")
        ));
    }
    *LOCALE.write().unwrap() = locale.clone();
    println!("Locale set to: {}", locale);

    // Update tray menu with new locale
    update_tray_menu(&app, &locale)?;
    Ok(())
}

/// Get current locale
#[command]
pub fn get_locale() -> String {
    LOCALE.read().unwrap().clone()
}

/// Update tray menu with localized text
fn update_tray_menu(app: &tauri::AppHandle, locale: &str) -> Result<(), String> {
    use tauri::menu::{Menu, MenuItem};

    let (show_text, quit_text) = get_tray_menu_text(locale);

    let show_item = MenuItem::with_id(app, "show", show_text, true, None::<&str>)
        .map_err(|e| e.to_string())?;
    let quit_item = MenuItem::with_id(app, "quit", quit_text, true, None::<&str>)
        .map_err(|e| e.to_string())?;
    let menu = Menu::with_items(app, &[&show_item, &quit_item]).map_err(|e| e.to_string())?;

    if let Some(tray) = app.tray_by_id("main") {
        tray.set_menu(Some(menu)).map_err(|e| e.to_string())?;
    }

    Ok(())
}

/// Set tray icon display mode (0=indicator only, 1=indicator+note name, 2=indicator+cents)
#[command]
pub fn set_tray_icon_mode(app: tauri::AppHandle, mode: u32) -> Result<(), String> {
    TRAY_ICON_MODE.store(mode.min(2), Ordering::SeqCst);
    println!("Tray icon mode set to: {}", mode);

    // Get last tuning info and immediately redraw icon
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

/// Set threshold ratio (range: 1.1 to 10.0)
#[command]
pub fn set_threshold(ratio: f32) -> Result<(), String> {
    let mut threshold = THRESHOLD_RATIO.lock().map_err(|e| e.to_string())?;
    *threshold = ratio.max(1.1).min(10.0);
    println!("Threshold set to: {:.2}", *threshold);
    Ok(())
}

/// Get current threshold value
#[command]
pub fn get_threshold() -> Result<f32, String> {
    let threshold = THRESHOLD_RATIO.lock().map_err(|e| e.to_string())?;
    Ok(*threshold)
}

/// Get audio device list
#[command]
pub fn get_audio_devices() -> Result<Vec<String>, String> {
    get_input_device_names()
}

/// Start monitoring audio input on the specified device
#[command]
pub fn start_listening(app: tauri::AppHandle, device_name: String) -> Result<(), String> {
    // Stop existing analysis thread
    STOP_FLAG.store(true, Ordering::SeqCst);
    thread::sleep(Duration::from_millis(150)); // Wait for thread to exit
    STOP_FLAG.store(false, Ordering::SeqCst);

    // Issue a new stream ID (to stop old threads)
    let current_stream_id = STREAM_ID.fetch_add(1, Ordering::SeqCst) + 1;
    println!("Starting stream ID: {}", current_stream_id);

    // Find device
    let device = find_device_by_name(&device_name)?;

    // Start audio stream
    let (buffer, sample_rate, channels) = start_audio_stream(&device)?;

    // Launch analysis thread
    run_analysis_thread(app, buffer, sample_rate, channels);

    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub device_name: Option<String>,
    pub threshold: Option<f32>,
    pub channel_mode: Option<u32>,
    pub tray_icon_mode: Option<u32>,
    // Settings for pitch reference and tuning
    pub pitch_mode: Option<String>, // "standard" | "custom" | "shift"
    pub custom_pitch: Option<f32>,  // 438.0-445.0
    pub tuning_shift: Option<i32>,  // -1 to -5
    pub drop_tuning_enabled: Option<bool>, // Drop tuning enabled/disabled
    pub drop_tuning_note: Option<String>, // "D" | "C#" | "C" | "B"
    pub theme_mode: Option<String>, // "system" | "light" | "dark"
    pub always_on_top: Option<bool>, // Always display window on top
    pub locale: Option<String>,     // "en" | "ja"
}

fn settings_path() -> PathBuf {
    // Same directory as exe
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
            always_on_top: None,
            locale: None,
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
