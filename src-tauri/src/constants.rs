use cpal::Stream;
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicBool, AtomicU32};
use std::sync::Mutex;

/// グローバルストリームの保持（dropされないようにする）
pub static STREAM: Lazy<Mutex<Option<Stream>>> = Lazy::new(|| Mutex::new(None));

/// 閾値設定（グローバル）
pub static THRESHOLD_RATIO: Lazy<Mutex<f32>> = Lazy::new(|| Mutex::new(2.0));

/// チャンネル選択（0=左, 1=右, 2=両方の平均）
pub static CHANNEL_MODE: AtomicU32 = AtomicU32::new(1); // デフォルトは右チャンネル

/// 解析スレッド制御用
pub static STREAM_ID: AtomicU32 = AtomicU32::new(0);
pub static STOP_FLAG: AtomicBool = AtomicBool::new(false);

/// FFTサイズ（高精度のため16384に増加、分解能: 約2.9Hz @48kHz）
pub const FFT_SIZE: usize = 16384;

/// ギター弦の標準チューニング周波数（Hz）
pub const GUITAR_FREQUENCIES: [f32; 6] = [
    82.41,  // E2
    110.0,  // A2
    146.83, // D3
    196.0,  // G3
    246.94, // B3
    329.63, // E4
];

/// ギター音判定の許容誤差（±15%）
pub const GUITAR_TOLERANCE: f32 = 0.15;

/// 周波数解析範囲（Hz）
pub const MIN_FREQUENCY: f32 = 75.0;
pub const MAX_FREQUENCY: f32 = 350.0;

/// RMS閾値（ノイズフロア以下はスキップ）
pub const RMS_THRESHOLD: f32 = 0.001;
