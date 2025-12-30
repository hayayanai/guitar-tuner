# guitar-tuner

[English README](README.md)

Tauri v2 + Vue 3 + Rust (cpal/rustfft) によるギターチューナーアプリ

## 機能

- 🎸 6弦の標準チューニングサポート (E2/A2/D3/G3/B3/E4)
- 🎼 高精度周波数検出 (FFT 16384 + ゼロパディング + ガウス補間)
- 📈 セントメーター (±50セント表示) とチューニング状態表示 (Perfect/Good/Off)
- 🎚️ 入力レベルメーター (-80dBから0dB)
- 🔊 チャンネル選択 (L/R/両方) - オーディオインターフェース互換
- ⚙️ 感度調整スライダー
- 🎵 基準ピッチ設定 (標準 A4=440Hz / カスタム 438-445Hz / チューニングシフト ±1半音)
- 🎸 6弦ドロップチューニングサポート (D/C#/C/B)
- 🎯 弦参照表示とアクティブ音名ハイライト
- 🌓 テーマモード (ライト/ダーク/システム連動)
- 📌 システムトレイ統合とバックグラウンド動作
- 🔄 GitHub Releasesからの自動アップデート

## 技術スタック

- **フロントエンド**: Vue 3 + TypeScript + Vite
- **バックエンド**: Rust + Tauri v2
- **音声処理**: cpal (音声入力) + rustfft (FFT分析)
- **ウィンドウ関数**: ブラックマン-ハリス (サイドローブ抑制)
- **周波数検出**: 基本周波数を優先する調和検出

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

### ビルド注意事項（アップデーター署名鍵）

本アプリは Tauri v2 Updater を使用します。署名済みアップデートをビルド・配布するには、以下の設定が必要です:

- [src-tauri/tauri.conf.json](src-tauri/tauri.conf.json) の `plugins.updater.pubkey` に、生成した公開鍵を設定する
- CI のリポジトリシークレット `TAURI_SIGNING_PRIVATE_KEY` に、秘密鍵の内容を設定する
  - 秘密鍵にパスワードを設定している場合は、`TAURI_SIGNING_PRIVATE_KEY_PASSWORD` も併せて設定（任意）

手順は [UPDATER_SETUP.md](UPDATER_SETUP.md) を参照してください。

### 開発環境での自動更新のテスト

開発者は、リリースを公開せずに開発モードで自動更新機能をテストできます。`TAURI_DEV_UPDATER_ENDPOINT` 環境変数を設定して、ローカルまたはステージングサーバーを指定します。

完全なセットアップ手順と例については、[docs/UPDATER_DEV_GUIDE.md](docs/UPDATER_DEV_GUIDE.md) を参照してください。

クイックリファレンス:

```powershell
# 署名鍵ペアの生成（公開鍵が表示されます）
npm run tauri signer generate -- -w .tauri\guitar-tuner.key

# 秘密鍵をクリップボードへコピー（Windows）
Get-Content $env:USERPROFILE\.tauri\guitar-tuner.key | Set-Clipboard
```

公開鍵を `tauri.conf.json` に設定し、秘密鍵を GitHub Secrets に登録したら、タグを付けてプッシュすると署名付きリリースが生成されます:

```bash
git tag v0.2.4
git push origin v0.2.4
```

## CI/CDとGitHub Actions

タグをプッシュすることで (例: `v0.2.0`)、Windowsインストーラーが自動的にビルドされ、GitHub Releasesにドラフトリリースが作成されます。

CIは `.mise.toml` のバージョンを参照して Node.js/Rust をセットアップします。

手動実行もActionsタブから可能です。

## 主要ファイル

| ファイル                    | 説明                                            |
| --------------------------- | ----------------------------------------------- |
| `src-tauri/src/lib.rs`      | Rustバックエンド（音声入力・FFT・イベント送信） |
| `src/App.vue`               | Vue UI（チューナー画面）                        |
| `src-tauri/tauri.conf.json` | Tauri設定                                       |

## コード整形・静的解析

### Linter（静的解析）

TypeScript/Vue コードの静的解析:

```bash
npm run lint
```

### Formatter（フォーマッタ）

コード整形:

```bash
npm run format
```

**prettier**: フォーマッタ
**oxlint**: 静的解析（TypeScript/JavaScript）
`eslint-plugin-oxlint`でESLintと併用可能
`eslint-plugin-vue`でVueファイルもESLint経由でlint可能

これらのチェックはCIでも自動実行されます。コミット前にローカルでも実行してください。

## ライセンス

MIT
