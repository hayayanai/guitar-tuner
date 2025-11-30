# guitar-tuner

Tauri + Vue + Rust (cpal/rustfft) ギターチューナーアプリ

## 概要
- Windowsタスクトレイ常駐型のギターチューナー
- Rust (cpal) で音声入力・FFT解析、VueでUI
- 入力デバイス選択、リアルタイム周波数・音名・セント表示

## 現状
- ✅ Rust側の構文エラーは解消済み
- ✅ ビルドは正常に通る
- ✅ 音声入力ストリームのコールバックが正常に動作
- ✅ FFT解析・周波数検出が動作
- ✅ フロントエンドへの周波数イベント送信が動作

## ビルド・実行
- `npm install`
- `mise install`
- `npm run tauri build` または `npm run tauri dev`

## 主要ファイル
- `src-tauri/src/lib.rs` ... Rustバックエンド（cpal/FFT/イベント）
- `src/App.vue` ... Vue UI
- `tauri.conf.json` ... Tauri設定

## TODO
- 構文エラーの根本解消（cpalストリーム作成部のクロージャ渡し方修正）
- 音声入力ストリームのコールバックが呼ばれることの確認
- 周波数解析・イベント送信の動作確認
- タスクトレイ機能・アイコン差し替え
