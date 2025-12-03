# guitar-tuner

[日本語版 README はこちら](README_JP.md)

A cross-platform guitar tuner app built with Tauri v2 + Vue 3 + Rust (cpal/rustfft).

## Features

- 🎸 Standard tuning support for 6 strings (E2/A2/D3/G3/B3/E4)
- 🎯 High-precision frequency detection (FFT 16384 + zero-padding + Gaussian interpolation)
- 📊 Cent meter (±50 cents display)
- 🎚️ Input level meter (-80dB to 0dB)
- 🔧 Channel selection (L/R/Both) - Audio interface compatible
- ⚙️ Sensitivity adjustment slider

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

## CI/CD with GitHub Actions

Pushing a tag (e.g., `v0.2.0`) automatically builds a Windows installer and creates a draft release on GitHub Releases.

CI uses `.mise.toml` to set up Node.js/Rust versions.

You can also trigger builds manually from the Actions tab.

## Project Structure

| File | Description |
|------|-------------|
| `src-tauri/src/lib.rs` | Rust backend (audio input, FFT, event emission) |
| `src/App.vue` | Vue UI (tuner screen) |
| `src-tauri/tauri.conf.json` | Tauri configuration |

## Code Formatting & Linting

- **prettier**: Code formatter
- **oxlint**: Fast linter for TypeScript/JavaScript
  - Used with `eslint-plugin-oxlint` for ESLint integration
  - `eslint-plugin-vue` for Vue file linting
  - Run `npm run lint` to lint all files

## License

MIT
