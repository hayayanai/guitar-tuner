# Auto-Updater Development Guide

This guide explains how to test the auto-updater functionality in development mode.

## Overview

The auto-updater is configured to work differently in development and production:

- **Production**: Uses GitHub Releases endpoint configured in `tauri.conf.json`
- **Development**: Can use a custom endpoint via environment variable

## Testing Auto-Update in Development

### Quick Start with Example Files

The fastest way to test the updater is using the provided example files:

1. Start the example HTTP server:
   ```bash
   # Using Python
   cd examples/updater-test
   python -m http.server 8080
   ```

2. In a new terminal, set the environment variable and run the app:
   ```bash
   $env:TAURI_DEV_UPDATER_ENDPOINT="http://localhost:8080/latest.json"
   npm run tauri dev
   ```

3. The app will check for updates on startup using the example `latest.json` file.

See [examples/updater-test/README.md](examples/updater-test/README.md) for more details.

### Option 1: Using a Local Update Server

1. Create a test update manifest file (`latest.json`):

```json
{
  "version": "0.2.7",
  "notes": "Test update for development",
  "pub_date": "2024-01-01T00:00:00Z",
  "platforms": {
    "windows-x86_64": {
      "signature": "dW50cnVzdGVkIGNvbW1lbnQ6IHNpZ25hdHVyZSBmcm9tIHRhdXJpIHNlY3JldCBrZXkKUldSWFpxNzhOR3FSWmtpVEJoSnJPU055N3QvTjE1VWJLbmdmOHcyekZkOEFxMHNUN1AzblpxMFcK",
      "url": "http://localhost:8080/guitar-tuner_0.2.7_x64-setup.nsis.zip"
    }
  }
}
```

2. Start a local HTTP server to serve the update files:

```bash
# Using Python
cd /path/to/update/files
python -m http.server 8080

# Or using Node.js http-server
npx http-server -p 8080
```

3. Set the development updater endpoint environment variable:

```bash
# Windows PowerShell
$env:TAURI_DEV_UPDATER_ENDPOINT="http://localhost:8080/latest.json"

# Windows CMD
set TAURI_DEV_UPDATER_ENDPOINT=http://localhost:8080/latest.json

# Linux/macOS
export TAURI_DEV_UPDATER_ENDPOINT=http://localhost:8080/latest.json
```

4. Run the app in development mode:

```bash
npm run tauri dev
```

### Option 2: Using a Staging Server

If you have a staging server, you can point to it:

```bash
export TAURI_DEV_UPDATER_ENDPOINT=https://staging.example.com/updates/latest.json
npm run tauri dev
```

## Creating Test Update Artifacts

To create a proper test update:

1. Build the app with the next version number:
   ```bash
   npm run bump
   npm run tauri build
   ```

2. The updater artifacts will be in `src-tauri/target/release/bundle/`:
   - `*.nsis.zip` - Signed installer for Windows
   - `*.nsis.zip.sig` - Signature file

3. Copy these files to your local server directory

4. Create the `latest.json` manifest with the correct version and signature

## Troubleshooting

### Update Check Fails

- Check the browser console (F12) for error messages
- Verify the endpoint is accessible: `curl http://localhost:8080/latest.json`
- Ensure CORS is enabled if using a remote server

### Update Not Detected

- Make sure the version in `latest.json` is higher than the current version
- Check that `TAURI_DEV_UPDATER_ENDPOINT` environment variable is set
- Look for log messages indicating the endpoint is being used

### Signature Verification Failed

- In development, you can test without proper signatures by creating a new keypair:
  ```bash
  npm run tauri signer generate -- -w ~/.tauri/dev.key
  ```
- Use the generated public key in your development builds

## Production Behavior

In production (release builds), the app will:
- Ignore the `TAURI_DEV_UPDATER_ENDPOINT` environment variable
- Use the production endpoint from `tauri.conf.json`:
  `https://github.com/hayayanai/guitar-tuner/releases/latest/download/latest.json`

This ensures that end users always receive official updates from GitHub Releases.

## Implementation Details

The development updater configuration is in `src-tauri/src/lib.rs`:

```rust
#[cfg(debug_assertions)]
{
    if let Ok(dev_endpoint) = std::env::var("TAURI_DEV_UPDATER_ENDPOINT") {
        if let Ok(url) = dev_endpoint.parse() {
            builder = builder.endpoints(vec![url]);
            log::info!("Using development updater endpoint: {}", dev_endpoint);
        }
    }
}
```

This conditional compilation ensures the development endpoint logic is only included in debug builds.
