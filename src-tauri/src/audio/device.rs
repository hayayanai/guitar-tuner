use cpal::traits::{DeviceTrait, HostTrait};

/// 利用可能なオーディオ入力デバイスの名前リストを取得
pub fn get_input_device_names() -> Result<Vec<String>, String> {
    let host = cpal::default_host();
    let devices = host.input_devices().map_err(|e| e.to_string())?;
    let mut names = Vec::new();
    for device in devices {
        if let Ok(name) = device.name() {
            names.push(name);
        }
    }
    Ok(names)
}

/// 指定された名前のオーディオデバイスを検索
pub fn find_device_by_name(device_name: &str) -> Result<cpal::Device, String> {
    let host = cpal::default_host();
    host.input_devices()
        .map_err(|e| e.to_string())?
        .find(|d| d.name().map(|n| n == device_name).unwrap_or(false))
        .ok_or_else(|| format!("デバイスが見つかりません: {}", device_name))
}
