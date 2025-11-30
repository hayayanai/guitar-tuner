use cpal::traits::{DeviceTrait, StreamTrait};
use std::sync::{Arc, Mutex};

use crate::constants::{FFT_SIZE, STREAM};

/// オーディオ入力ストリームを開始し、バッファにデータを蓄積
pub fn start_audio_stream(
    device: &cpal::Device,
) -> Result<(Arc<Mutex<Vec<f32>>>, usize, usize), String> {
    let config = device.default_input_config().map_err(|e| e.to_string())?;
    let sample_rate = config.sample_rate().0 as usize;
    let channels = config.channels() as usize;
    let buffer_size = FFT_SIZE * channels;
    let buffer = Arc::new(Mutex::new(Vec::<f32>::with_capacity(buffer_size)));
    let buffer_clone = buffer.clone();

    println!(
        "Audio config: sample_rate={}, channels={}, fft_size={}",
        sample_rate, channels, FFT_SIZE
    );

    let err_fn = |err| eprintln!("Stream error: {}", err);

    let stream = device
        .build_input_stream(
            &config.clone().into(),
            move |data: &[f32], _| {
                let mut buf = buffer_clone.lock().unwrap();
                buf.extend_from_slice(data);
                if buf.len() > buffer_size {
                    let len = buf.len();
                    buf.drain(..len - buffer_size);
                }
            },
            err_fn,
            None,
        )
        .map_err(|e| e.to_string())?;

    stream
        .play()
        .map_err(|e| format!("ストリーム開始失敗: {}", e))?;

    // ストリームをグローバルに保持（dropされないようにする）
    {
        let mut global_stream = STREAM.lock().unwrap();
        *global_stream = Some(stream);
    }

    Ok((buffer, sample_rate, channels))
}
