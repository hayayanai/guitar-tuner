use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;
use tauri::command;

use crate::audio::{find_device_by_name, get_input_device_names, start_audio_stream};
use crate::constants::{CHANNEL_MODE, STOP_FLAG, STREAM_ID, THRESHOLD_RATIO};
use crate::dsp::run_analysis_thread;

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
