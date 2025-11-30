# ギターチューナー TODO

## ✅ 完了

### 開発環境
- [x] npm install
- [x] mise install
- [x] npm run tauri dev 動作確認

### バックエンド (Rust)
- [x] cpal / rustfft 追加
- [x] get_audio_devices コマンド
- [x] start_listening コマンド
- [x] FFT解析（Blackman-Harris窓 + ゼロパディング + ガウシアン補間）
- [x] 倍音検出（基音優先）
- [x] 周波数イベント送信
- [x] レベルメーター（input_level イベント）
- [x] チャンネル選択（L/R/両方）
- [x] 感度調整

### フロントエンド (Vue)
- [x] デバイス選択UI
- [x] 音名・周波数表示
- [x] セントメーター（±50セント）
- [x] レベルメーター
- [x] チャンネル選択ボタン
- [x] 感度スライダー

## 🔲 未実装

### アプリ仕上げ
- [ ] タスクトレイ機能（tauri.conf.json）
- [ ] アイコン差し替え（src-tauri/icons）
- [ ] リリースビルド（npm run tauri build）
