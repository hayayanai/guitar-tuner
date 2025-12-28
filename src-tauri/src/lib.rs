mod audio;
mod commands;
mod constants;
mod dsp;

use commands::{
    get_audio_devices, get_channel_mode, get_locale, get_settings, get_threshold,
    get_tray_icon_mode, set_always_on_top, set_channel_mode, set_custom_pitch, set_drop_tuning,
    set_locale, set_pitch_mode, set_settings, set_threshold, set_tray_icon_mode, set_tuning_shift,
    start_listening,
};

pub fn run() {
    use tauri::image::Image;
    use tauri::menu::{Menu, MenuItem};
    use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
    use tauri::{Manager, WindowEvent};

    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .invoke_handler(tauri::generate_handler![
            get_audio_devices,
            start_listening,
            set_threshold,
            get_threshold,
            set_channel_mode,
            get_channel_mode,
            set_tray_icon_mode,
            get_tray_icon_mode,
            set_settings,
            get_settings,
            set_pitch_mode,
            set_custom_pitch,
            set_tuning_shift,
            set_drop_tuning,
            set_always_on_top,
            set_locale,
            get_locale
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();

            // Load locale from settings
            use crate::commands::get_settings;
            let locale = match get_settings() {
                Ok(settings) => settings.locale.unwrap_or_else(|| "en".to_string()),
                Err(_) => "en".to_string(),
            };
            *commands::LOCALE.write().unwrap() = locale.clone();

            // トレイメニュー作成
            let (show_text, quit_text) = match locale.as_str() {
                "ja" => ("ウィンドウを表示", "終了"),
                _ => ("Show Window", "Quit"),
            };
            let show_item = MenuItem::with_id(app, "show", show_text, true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", quit_text, true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_item, &quit_item])?;

            // トレイアイコン作成（アイコンファイル読み込み）
            let icon = Image::from_path("icons/icon.ico").unwrap_or_else(|_| {
                // フォールバック: 組み込みデフォルト（32x32 PNG）
                Image::from_path("icons/32x32.png").unwrap_or_else(|_| {
                    // 最終フォールバック: 空のアイコン
                    Image::new_owned(vec![0u8; 32 * 32 * 4], 32, 32)
                })
            });

            TrayIconBuilder::with_id("main")
                .icon(icon)
                .menu(&menu)
                .tooltip("Guitar Tuner")
                .on_menu_event(move |app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.unminimize();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        std::process::exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(move |tray, event| {
                    match event {
                        TrayIconEvent::Click {
                            button: MouseButton::Left,
                            button_state: MouseButtonState::Up,
                            ..
                        } => {
                            // 左クリックでウィンドウを表示
                            if let Some(window) = tray.app_handle().get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.unminimize();
                                let _ = window.set_focus();
                            }
                        }
                        _ => {}
                    }
                })
                .build(app)?;

            // ウィンドウの閉じるボタンで隠す（終了しない）
            let window = app.get_webview_window("main").unwrap();
            window.on_window_event(move |event| {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    if let Some(win) = app_handle.get_webview_window("main") {
                        let _ = win.hide();
                    }
                }
            });

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
