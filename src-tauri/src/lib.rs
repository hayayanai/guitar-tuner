mod audio;
mod commands;
mod constants;
mod dsp;

use commands::{
    get_audio_devices, get_channel_mode, get_threshold, set_channel_mode, set_threshold,
    start_listening,
};

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_audio_devices,
            start_listening,
            set_threshold,
            get_threshold,
            set_channel_mode,
            get_channel_mode
        ])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
