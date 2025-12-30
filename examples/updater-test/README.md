# Updater Test Example

This directory contains example files for testing the auto-updater in development mode.

## Quick Start

1. **Edit latest.json** if needed:
   - Update the `version` to be higher than your current app version
   - Update the `url` to point to your test installer
   - Update the `signature` if you have generated new signing keys

2. **Start a local HTTP server**:

   Using Node.js:
   ```bash
   npx http-server -p 8080 examples/updater-test
   ```

3. **Set the environment variable**:

   Windows PowerShell:
   ```powershell
   $env:TAURI_DEV_UPDATER_ENDPOINT="http://localhost:8080/latest.json"
   ```

   Windows CMD:
   ```cmd
   set TAURI_DEV_UPDATER_ENDPOINT=http://localhost:8080/latest.json
   ```

4. **Run the app**:
   ```bash
   npm run tauri dev
   ```

5. The app will check for updates on startup. If a newer version is found in `latest.json`, you'll see an update notification.

## Files

- `latest.json` - Sample update manifest with example data
  - **IMPORTANT**: The signature field contains `INVALID_DEV_SIGNATURE_DO_NOT_USE_IN_PRODUCTION` which is NOT a valid signature
  - This manifest is for testing the update check and notification UI only
  - For actual testing with signature verification and installation, you need to:
    1. Build a release version with a higher version number using your signing keys
    2. Copy the actual signature from the generated `.sig` file
    3. Replace the invalid signature in `latest.json` with the real signature
  - See [docs/UPDATER_DEV_GUIDE.md](../../docs/UPDATER_DEV_GUIDE.md) for details on generating signing keys
- `README.md` - This file

## Notes

- The signature in the example `latest.json` is a placeholder. For actual testing with signature verification, you'll need to:
  1. Build a release version with a higher version number
  2. Use the actual signature from the `.sig` file
  3. Ensure the installer file is available at the specified URL

- For testing without signature verification (development only), you can modify the updater configuration

- The example assumes port 8080. You can use any port, just update the URL accordingly.

For more detailed information, see [docs/UPDATER_DEV_GUIDE.md](../../docs/UPDATER_DEV_GUIDE.md).
