use cpal::Stream;
use once_cell::sync::Lazy;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU32};
use std::sync::{Mutex, RwLock};
use std::time::Instant;

/// グローバルストリームの保持（dropされないようにする）
pub static STREAM: Lazy<Mutex<Option<Stream>>> = Lazy::new(|| Mutex::new(None));

/// 閾値設定（グローバル）
pub static THRESHOLD_RATIO: Lazy<Mutex<f32>> = Lazy::new(|| Mutex::new(2.0));

/// チャンネル選択（0=左, 1=右, 2=両方の平均）
pub static CHANNEL_MODE: AtomicU32 = AtomicU32::new(1); // デフォルトは右チャンネル

/// トレイアイコン表示モード（0=インジケーターのみ, 1=インジケーター+音名, 2=インジケーター+セント値）
pub static TRAY_ICON_MODE: AtomicU32 = AtomicU32::new(1); // デフォルトは音名表示

/// 基準ピッチモード (0=standard, 1=custom, 2=shift)
pub static PITCH_MODE: AtomicU32 = AtomicU32::new(0);

/// カスタム基準ピッチ (Hz) - デフォルト440.0
pub static CUSTOM_PITCH: Lazy<RwLock<f32>> = Lazy::new(|| RwLock::new(440.0));

/// チューニングシフト（半音数、負の値）
pub static TUNING_SHIFT: AtomicI32 = AtomicI32::new(0);

/// 6弦ドロップチューニング有効フラグ
pub static DROP_TUNING_ENABLED: AtomicBool = AtomicBool::new(false);

/// 6弦ドロップ音名 (0=D, 1=C#, 2=C, 3=B)
pub static DROP_TUNING_NOTE: AtomicU32 = AtomicU32::new(0);

/// 解析スレッド制御用
pub static STREAM_ID: AtomicU32 = AtomicU32::new(0);
pub static STOP_FLAG: AtomicBool = AtomicBool::new(false);

/// 最後に検出されたチューニング情報（トレイアイコン用）
pub static LAST_TUNING_INFO: Lazy<Mutex<TuningInfo>> =
    Lazy::new(|| Mutex::new(TuningInfo::default()));

/// トレイアイコンの状態管理（ちらつき防止用）
pub static TRAY_ICON_STATE: Lazy<Mutex<TrayIconState>> =
    Lazy::new(|| Mutex::new(TrayIconState::new()));

/// チューニング情報構造体
#[derive(Clone, Default)]
pub struct TuningInfo {
    pub note_name: String,
    pub frequency: f32,
    pub cents: f32,
}

/// トレイアイコンの状態（ちらつき防止のためのキャッシュ）
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TuningColor {
    Green,  // チューニング良好 (|cents| <= TUNING_GREEN_THRESHOLD)
    Yellow, // 少しずれ (TUNING_GREEN_THRESHOLD < |cents| < TUNING_RED_THRESHOLD)
    Red,    // 大きくずれ (|cents| >= TUNING_RED_THRESHOLD)
}

pub struct TrayIconState {
    pub last_color: Option<TuningColor>,
    pub last_indicator_pos: Option<i32>, // インジケーターの位置（-50〜+50の整数）
    pub last_update_time: Option<Instant>,
}

impl TrayIconState {
    pub fn new() -> Self {
        Self {
            last_color: None,
            last_indicator_pos: None,
            last_update_time: None,
        }
    }

    /// 更新が必要かどうか判定（デバウンス + 変化検出）
    pub fn should_update(&self, new_color: TuningColor, new_pos: i32) -> bool {
        // 初回は常に更新
        let Some(last_time) = self.last_update_time else {
            return true;
        };

        // 最小更新間隔（200ms）
        const MIN_UPDATE_INTERVAL_MS: u64 = 200;
        if last_time.elapsed().as_millis() < MIN_UPDATE_INTERVAL_MS as u128 {
            return false;
        }

        // 色が変わった場合は更新
        if self.last_color != Some(new_color) {
            return true;
        }

        // インジケーター位置が大きく変わった場合は更新（3以上の変化）
        if let Some(last_pos) = self.last_indicator_pos {
            if (new_pos - last_pos).abs() >= 3 {
                return true;
            }
        } else {
            return true;
        }

        false
    }

    /// 状態を更新
    pub fn update(&mut self, color: TuningColor, pos: i32) {
        self.last_color = Some(color);
        self.last_indicator_pos = Some(pos);
        self.last_update_time = Some(Instant::now());
    }

    /// リセット
    pub fn reset(&mut self) {
        self.last_color = None;
        self.last_indicator_pos = None;
        self.last_update_time = None;
    }
}

/// 色判定の閾値（セント）
pub const TUNING_GREEN_THRESHOLD: f32 = 3.0;
pub const TUNING_RED_THRESHOLD: f32 = 10.0;
pub const TUNING_HYSTERESIS: f32 = 1.0;

/// ヒステリシス付きで色を判定（境界付近でのちらつき防止）
/// 緑: ±3セント以下、黄: ±3〜10セント未満、赤: ±10セント以上
pub fn determine_color_with_hysteresis(
    cents: f32,
    current_color: Option<TuningColor>,
) -> TuningColor {
    let abs_cents = cents.abs();

    match current_color {
        Some(TuningColor::Green) => {
            // 緑から黄色への移行は3+ヒステリシス未満ならGreen、以上ならYellow/Red
            if abs_cents <= TUNING_GREEN_THRESHOLD + TUNING_HYSTERESIS {
                TuningColor::Green
            } else if abs_cents >= TUNING_RED_THRESHOLD {
                TuningColor::Red
            } else {
                TuningColor::Yellow
            }
        }
        Some(TuningColor::Yellow) => {
            // 黄色から緑への移行は3-ヒステリシス以下、赤への移行は10+ヒステリシス以上
            if abs_cents <= TUNING_GREEN_THRESHOLD - TUNING_HYSTERESIS {
                TuningColor::Green
            } else if abs_cents >= TUNING_RED_THRESHOLD + TUNING_HYSTERESIS {
                TuningColor::Red
            } else {
                TuningColor::Yellow
            }
        }
        Some(TuningColor::Red) => {
            // 赤から黄色への移行は10-ヒステリシス未満
            if abs_cents < TUNING_RED_THRESHOLD - TUNING_HYSTERESIS {
                if abs_cents <= TUNING_GREEN_THRESHOLD - TUNING_HYSTERESIS {
                    TuningColor::Green
                } else {
                    TuningColor::Yellow
                }
            } else {
                TuningColor::Red
            }
        }
        None => {
            // 初期状態
            if abs_cents <= TUNING_GREEN_THRESHOLD {
                TuningColor::Green
            } else if abs_cents < TUNING_RED_THRESHOLD {
                TuningColor::Yellow
            } else {
                TuningColor::Red
            }
        }
    }
}

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
