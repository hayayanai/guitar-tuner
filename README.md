# guitar-tuner

[日本語版 README](README_JP.md)

A cross-platform guitar tuner app built with Tauri v2 + Vue 3 + Rust (cpal/rustfft).

## Features

- 🎸 Standard 6-string tuning support (E2/A2/D3/G3/B3/E4)
- 🎼 High-precision frequency detection (FFT 16384 + zero-padding + Gaussian interpolation)
- 📈 Cent meter (±50 cents display) with tuning status (Perfect/Good/Off)
- 🎚️ Input level meter (-80dB to 0dB)
- 🔊 Channel selection (L/R/Both) - Audio interface compatible
- ⚙️ Sensitivity adjustment slider
- 🎵 Reference pitch settings (Standard A4=440Hz / Custom 438-445Hz / Tuning shift ±1 semitone)
- 🎸 6th string drop tuning support (D/C#/C/B)
- 🎯 Visual string reference with active note highlighting
- 🌓 Theme mode (Light/Dark/System)
- 📌 System tray integration with background operation
- 🔄 Auto-update from GitHub Releases

## Tech Stack

- **Frontend**: Vue 3 + TypeScript + Vite
- **Backend**: Rust + Tauri v2
- **Audio Processing**: cpal (audio input) + rustfft (FFT analysis)
- **Window Function**: Blackman-Harris (sidelobe suppression)
- **Frequency Detection**: Harmonic detection prioritizing fundamental frequency

## Prerequisites

- **Node.js 24 LTS** (managed via `.mise.toml`, CI auto-configured)
- **Rust stable** (managed via `.mise.toml`, CI auto-configured)
- **Visual Studio 2022** with "Desktop development with C++" workload (required for Rust/cpal build)
  - Windows: Install via `winget install Microsoft.VisualStudio.2022.Community`
  - Enable "Desktop development with C++" during installation

## Getting Started

```bash
# Install Node.js/Rust versions (using mise)
mise install

# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for release
npm run tauri build
```

### Build Notes (Updater Signing Keys)

This app uses the Tauri v2 Updater. Building signed update artifacts requires:

- Set `plugins.updater.pubkey` in [src-tauri/tauri.conf.json](src-tauri/tauri.conf.json) to your generated public key.
- Configure CI secret `TAURI_SIGNING_PRIVATE_KEY` with your private key contents.
  - If your private key is password-protected, also set `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` (optional).

See [UPDATER_SETUP.md](UPDATER_SETUP.md) for step-by-step setup.

### Testing Auto-Update in Development

Developers can test the auto-updater in development mode without publishing releases. The auto-updater is configured to work differently in development and production:

- **Production**: Uses GitHub Releases endpoint configured in `tauri.conf.json`
- **Development**: Can use a custom endpoint via environment variable

#### Quick Start with Example Files

The fastest way to test the updater is using the provided example files in `examples/updater-test/`:

1. Start the example HTTP server:
   ```bash
   npx http-server -p 8080 examples/updater-test
   ```

2. In a new terminal, set the environment variable and run the app:
   ```bash
   # Windows PowerShell
   $env:TAURI_DEV_UPDATER_ENDPOINT="http://localhost:8080/latest.json"
   
   # Windows CMD
   set TAURI_DEV_UPDATER_ENDPOINT=http://localhost:8080/latest.json
   
   # Linux/macOS
   export TAURI_DEV_UPDATER_ENDPOINT=http://localhost:8080/latest.json
   
   # Run the app
   npm run tauri dev
   ```

3. The app will check for updates on startup using the example `latest.json` file.

**Note**: The example signature in `examples/updater-test/latest.json` is `INVALID_DEV_SIGNATURE_DO_NOT_USE_IN_PRODUCTION` for testing the UI only. For full signature verification testing, build a release with proper signing keys.

#### Creating Your Own Test Update

To create a proper test update with signature verification:

1. Build the app with the next version number:
   ```bash
   npm run bump
   npm run tauri build
   ```

2. The updater artifacts will be in `src-tauri/target/release/bundle/`:
   - `*.nsis.zip` - Signed installer for Windows
   - `*.nsis.zip.sig` - Signature file

3. Create your own `latest.json` manifest:
   ```json
   {
     "version": "0.2.7",
     "notes": "Test update for development",
     "pub_date": "2024-01-01T00:00:00Z",
     "platforms": {
       "windows-x86_64": {
         "signature": "PASTE_ACTUAL_SIGNATURE_HERE",
         "url": "http://localhost:8080/guitar-tuner_0.2.7_x64-setup.nsis.zip"
       }
     }
   }
   ```

4. Start a local HTTP server and serve your update files:
   ```bash
   npx http-server -p 8080 /path/to/update/files
   ```

#### Using a Staging Server

If you have a staging server, point to it directly:

```bash
export TAURI_DEV_UPDATER_ENDPOINT=https://staging.example.com/updates/latest.json
npm run tauri dev
```

#### Troubleshooting

**Update Check Fails:**
- Check the browser console (F12) for error messages
- Verify the endpoint is accessible: `curl http://localhost:8080/latest.json`
- Ensure CORS is enabled if using a remote server

**Update Not Detected:**
- Ensure version in `latest.json` is higher than the current version
- Check that `TAURI_DEV_UPDATER_ENDPOINT` environment variable is set
- Look for log messages indicating the endpoint is being used

**Signature Verification Failed:**
- For development testing without proper signatures, create a new keypair:
  ```bash
  npm run tauri signer generate -- -w ~/.tauri/dev.key
  ```

#### Production Behavior

In production (release builds), the app will:
- Ignore the `TAURI_DEV_UPDATER_ENDPOINT` environment variable
- Use the production endpoint from `tauri.conf.json`: `https://github.com/hayayanai/guitar-tuner/releases/latest/download/latest.json`

This ensures that end users always receive official updates from GitHub Releases.

Quick reference:

```powershell
# Generate signing key pair (prints public key)
npm run tauri signer generate -- -w .tauri\guitar-tuner.key

# Copy private key to clipboard (Windows)
Get-Content $env:USERPROFILE\.tauri\guitar-tuner.key | Set-Clipboard
```

After setting the public key in `tauri.conf.json` and the private key in GitHub Secrets, tag and push to trigger a signed release:

```bash
git tag v0.2.4
git push origin v0.2.4
```

## CI/CD with GitHub Actions

Pushing a tag (e.g., `v0.2.0`) automatically builds a Windows installer and creates a draft release on GitHub Releases.

CI uses `.mise.toml` to set up Node.js/Rust versions.

You can also trigger builds manually from the Actions tab.

### Bump Version Workflow Setup

The `bump-version.yml` workflow requires a Personal Access Token (PAT) to create pull requests:

1. Create a PAT with `pull_requests: write` and `contents: write` permissions at https://github.com/settings/tokens
2. Add it as a repository secret named `PAT_TOKEN` in Settings → Secrets and variables → Actions

Without this token, the workflow will fail with "GitHub Actions is not permitted to create or approve pull requests".

## Project Structure

| File                        | Description                                     |
| --------------------------- | ----------------------------------------------- |
| `src-tauri/src/lib.rs`      | Rust backend (audio input, FFT, event emission) |
| `src/App.vue`               | Vue UI (tuner screen)                           |
| `src-tauri/tauri.conf.json` | Tauri configuration                             |

## Code Formatting & Linting

### Linter

TypeScript/Vue code static analysis:

```bash
npm run lint
```

### Formatter

Code formatting:

```bash
npm run format
```

**prettier**: Code formatter
**oxlint**: Fast linter for TypeScript/JavaScript
`eslint-plugin-oxlint` for ESLint integration
`eslint-plugin-vue` for Vue file linting

These checks are also run automatically in CI. Please run them locally before committing.

## License

MIT
