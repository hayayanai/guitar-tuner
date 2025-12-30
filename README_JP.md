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

開発者は、リリースを公開せずに開発モードで自動更新機能をテストできます。自動更新は開発環境と本番環境で異なる設定を使用します：

- **本番環境**: `tauri.conf.json` で設定された GitHub Releases エンドポイントを使用
- **開発環境**: 環境変数でカスタムエンドポイントを使用可能

#### サンプルファイルを使ったクイックスタート

`examples/updater-test/` にあるサンプルファイルを使用するのが最も簡単です：

1. サンプル用の HTTP サーバーを起動：
   ```bash
   npx http-server -p 8080 examples/updater-test
   ```

2. 別のターミナルで環境変数を設定してアプリを実行：
   ```bash
   # Windows PowerShell
   $env:TAURI_DEV_UPDATER_ENDPOINT="http://localhost:8080/latest.json"
   
   # Windows CMD
   set TAURI_DEV_UPDATER_ENDPOINT=http://localhost:8080/latest.json
   
   # Linux/macOS
   export TAURI_DEV_UPDATER_ENDPOINT=http://localhost:8080/latest.json
   
   # アプリを実行
   npm run tauri dev
   ```

3. 起動時にサンプル `latest.json` ファイルを使用して更新チェックが実行されます。

**注意**: `examples/updater-test/latest.json` のサンプル署名は `INVALID_DEV_SIGNATURE_DO_NOT_USE_IN_PRODUCTION` で、UI テスト用です。完全な署名検証テストには、適切な署名鍵でリリースビルドを作成してください。

#### 独自のテスト更新を作成

署名検証付きの適切なテスト更新を作成するには：

1. 次のバージョン番号でアプリをビルド：
   ```bash
   npm run bump
   npm run tauri build
   ```

2. アップデート用の成果物は `src-tauri/target/release/bundle/` に生成されます：
   - `*.nsis.zip` - Windows 用署名済みインストーラー
   - `*.nsis.zip.sig` - 署名ファイル

3. 独自の `latest.json` マニフェストを作成：
   ```json
   {
     "version": "0.2.7",
     "notes": "開発用テストアップデート",
     "pub_date": "2024-01-01T00:00:00Z",
     "platforms": {
       "windows-x86_64": {
         "signature": "実際の署名をここに貼り付け",
         "url": "http://localhost:8080/guitar-tuner_0.2.7_x64-setup.nsis.zip"
       }
     }
   }
   ```

4. ローカル HTTP サーバーを起動して更新ファイルを配信：
   ```bash
   npx http-server -p 8080 /path/to/update/files
   ```

#### ステージングサーバーを使用

ステージングサーバーがある場合は、直接指定できます：

```bash
export TAURI_DEV_UPDATER_ENDPOINT=https://staging.example.com/updates/latest.json
npm run tauri dev
```

#### トラブルシューティング

**更新チェックが失敗する場合:**
- ブラウザコンソール（F12）でエラーメッセージを確認
- エンドポイントにアクセス可能か確認: `curl http://localhost:8080/latest.json`
- リモートサーバーを使用する場合は CORS が有効か確認

**更新が検出されない場合:**
- `latest.json` のバージョンが現在のバージョンより高いことを確認
- `TAURI_DEV_UPDATER_ENDPOINT` 環境変数が設定されているか確認
- エンドポイントが使用されていることを示すログメッセージを確認

**署名検証が失敗する場合:**
- 開発環境で適切な署名なしでテストする場合は、新しいキーペアを作成：
  ```bash
  npm run tauri signer generate -- -w ~/.tauri/dev.key
  ```

#### 本番環境での動作

本番環境（リリースビルド）では、アプリは：
- `TAURI_DEV_UPDATER_ENDPOINT` 環境変数を無視
- `tauri.conf.json` の本番エンドポイントを使用: `https://github.com/hayayanai/guitar-tuner/releases/latest/download/latest.json`

これにより、エンドユーザーは常に GitHub Releases から公式アップデートを受け取ることが保証されます。

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
