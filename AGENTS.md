# AGENTS.md

AI/自動化エージェント向けのプロジェクト情報です。

## プロジェクト概要
Tauri v2 + Vue 3 + Rust によるギターチューナーアプリ（Windows向け）

## アーキテクチャ

### Rustバックエンド (`src-tauri/src/lib.rs`)
- **cpal**: 音声入力ストリーム（48kHz, ステレオ）
- **rustfft**: FFT解析
- **グローバル状態**: `STREAM`, `THRESHOLD_RATIO`, `CHANNEL_MODE`, `STOP_FLAG`
- **Tauriコマンド**: `get_audio_devices`, `start_listening`, `set_threshold`, `set_channel_mode`
- **イベント**: `frequency`, `raw_frequency`, `input_level`

### 音声処理パイプライン
1. cpalで音声入力（48kHz, ステレオ）
2. チャンネル選択（L/R/両方）
3. Blackman-Harris窓関数適用
4. 2倍ゼロパディング（FFT 16384 → 32768）
5. FFT実行、パワースペクトル計算（75-350Hz範囲）
6. ガウシアン補間でピーク周波数推定
7. 倍音検出（1/2, 1/3, 1/4をチェック）で基音優先
8. ギター音フィルタ（E2〜E4の±15%範囲）
9. 中央値フィルタで安定化
10. フロントエンドへイベント送信

### Vueフロントエンド (`src/App.vue`)
- デバイス選択ドロップダウン
- 音名・周波数表示（現在値と目標値）
- セントメーター（±50セント、水平バー）
- レベルメーター（-80dB〜0dB）
- チャンネル選択ボタン（L/L+R/R）
- 感度スライダー

## 注意点

### Rustクロージャの書き方
```rust
// ✅ 正しい
let buffer_clone = buffer.clone();
device.build_input_stream(
  &config,
  move |data: &[f32], _| { /* ... */ },
  err_fn,
  None
)

// ❌ 間違い（余計なブロック）
device.build_input_stream(
  &config,
  { move |data: &[f32], _| { /* ... */ } },  // ブロックで囲まない
  err_fn,
  None
)
```

### ストリームの保持
cpalのストリームはdropされるとコールバックが停止するため、グローバルに保持：
```rust
static STREAM: Lazy<Mutex<Option<Stream>>> = Lazy::new(|| Mutex::new(None));
```

### コールバックが呼ばれない場合
- Windowsの「サウンド設定」→「録音」で排他モードをOFF
- 他のDAW/録音アプリを閉じる
- 別の入力デバイスで試す

## 開発コマンド
```bash
npm install        # 依存関係インストール
mise install       # Node.js/Rustバージョン管理
npm run tauri dev  # 開発モード
npm run tauri build # リリースビルド
```
