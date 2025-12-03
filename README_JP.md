# guitar-tuner

[English README](README.md)

Tauri v2 + Vue 3 + Rust (cpal/rustfft) によるギターチューナーアプリ

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

## 必要な環境・依存

- **Node.js 24 LTS**（`.mise.toml`で管理、CIも自動対応）
- **Rust stable**（`.mise.toml`で管理、CIも自動対応）
- **Visual Studio 2022** の「C++によるデスクトップ開発」ワークロード（Rust/cpal依存のビルドに必要）
  - Windows: `winget install Microsoft.VisualStudio.2022.Community` などでインストール
  - インストール時に「C++によるデスクトップ開発」を有効化

## セットアップ

```bash
# Node.js/Rustバージョン自動管理（mise使用）
mise install

# 依存関係インストール
npm install

# 開発モード
npm run tauri dev

# リリースビルド
npm run tauri build
```

## GitHub Actionsによる自動リリース

タグ（例: `v0.2.0`）をpushすると、Windows用インストーラーがGitHub Releasesにドラフトとして自動生成されます。

CIは `.mise.toml` のバージョンを参照して Node.js/Rust をセットアップします。

手動実行もActionsタブから可能です。

## 主要ファイル

| ファイル | 説明 |
|---------|------|
| `src-tauri/src/lib.rs` | Rustバックエンド（音声入力・FFT・イベント送信） |
| `src/App.vue` | Vue UI（チューナー画面） |
| `src-tauri/tauri.conf.json` | Tauri設定 |

## コード整形・静的解析

- **prettier**: フォーマッタ
- **oxlint**: 静的解析（TypeScript/JavaScript）
  - `eslint-plugin-oxlint`でESLintと併用可能
  - `eslint-plugin-vue`でVueファイルもESLint経由でlint可能
  - `npm run lint` で全ファイルlint

## ライセンス

MIT
