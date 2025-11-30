# guitar-tuner

Tauri + Vue + Rust (cpal/rustfft) ギターチューナーアプリ

## 機能
- 🎸 レギュラーチューニング6弦対応（E2/A2/D3/G3/B3/E4）
- 🎯 高精度周波数検出（FFT 16384 + ゼロパディング + ガウシアン補間）
- 📊 セントメーター（±50セント表示）
- 🎚️ 入力レベルメーター（-80dB〜0dB）
- 🔧 チャンネル選択（L/R/両方）- オーディオインターフェース対応
- ⚙️ 感度調整スライダー

## 技術スタック
- **フロントエンド**: Vue 3 + TypeScript + Vite
- **バックエンド**: Rust + Tauri v2
- **音声処理**: cpal（音声入力）+ rustfft（FFT解析）
- **窓関数**: Blackman-Harris（サイドローブ抑制）
- **周波数検出**: 倍音検出で基音を優先

## ビルド・実行

```bash
npm install
mise install
npm run tauri dev    # 開発モード
npm run tauri build  # リリースビルド
```

## 主要ファイル
| ファイル | 説明 |
|---------|------|
| `src-tauri/src/lib.rs` | Rustバックエンド（音声入力・FFT・イベント送信） |
| `src/App.vue` | Vue UI（チューナー画面） |
| `src-tauri/tauri.conf.json` | Tauri設定 |

## ライセンス
MIT
