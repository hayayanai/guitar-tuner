import { ref, onMounted, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { ChannelMode, PitchMode } from "../types";

export type Settings = {
  device_name?: string;
  threshold?: number;
  channel_mode?: number;
  tray_icon_mode?: number;
  pitch_mode?: PitchMode;
  custom_pitch?: number;
  tuning_shift?: number;
  drop_tuning_enabled?: boolean;
  drop_tuning_note?: string;
};

/**
 * オーディオデバイスと設定を管理するComposable
 */
export function useAudioDevice() {
  const devices = ref<string[]>([]);
  const selectedDevice = ref("");
  const loading = ref(true);
  const error = ref("");
  const listenStatus = ref("");
  const frequency = ref<number | null>(null);
  const rawFrequency = ref<number | null>(null);
  const threshold = ref(2.0);
  const inputLevel = ref(0);
  const channelMode = ref<ChannelMode>(1); // 0=左, 1=右, 2=平均

  // 新しい設定項目
  const pitchMode = ref<PitchMode>("standard");
  const customPitch = ref(440);
  const tuningShift = ref(-1);
  const dropEnabled = ref(false);
  const dropNote = ref("D");

  async function updateThreshold(value: number) {
    threshold.value = value;
    await invoke("set_threshold", { ratio: value });
    await saveSettings({ threshold: value });
  }

  async function updateChannelMode(mode: ChannelMode) {
    channelMode.value = mode;
    await invoke("set_channel_mode", { mode });
    await saveSettings({ channel_mode: mode });
  }

  async function startListening(device: string) {
    try {
      listenStatus.value = "Starting...";
      await invoke("start_listening", { deviceName: device });
      listenStatus.value = `Listening: ${device}`;
      await saveSettings({ device_name: device });
    } catch (e: unknown) {
      listenStatus.value = "Failed: " + (e?.toString() ?? "");
    }
  }

  async function saveSettings(partial: Partial<Settings>) {
    // 既存設定を取得してマージ
    let current: Settings = {};
    try {
      current = await invoke<Settings>("get_settings");
    } catch {}
    const merged = { ...current, ...partial };
    await invoke("set_settings", { settings: merged });
  }

  // 設定変更の監視と反映
  watch(pitchMode, async (mode) => {
    const modeVal = mode === "standard" ? 0 : mode === "custom" ? 1 : 2;
    await invoke("set_pitch_mode", { mode: modeVal });
    await saveSettings({ pitch_mode: mode });
  });

  watch(customPitch, async (pitch) => {
    if (pitch >= 438 && pitch <= 445) {
      await invoke("set_custom_pitch", { pitch: Number(pitch) });
      await saveSettings({ custom_pitch: Number(pitch) });
    }
  });

  watch(tuningShift, async (shift) => {
    await invoke("set_tuning_shift", { semitones: Number(shift) });
    await saveSettings({ tuning_shift: Number(shift) });
  });

  watch([dropEnabled, dropNote], async ([enabled, note]) => {
    const noteVal = note === "D" ? 0 : note === "C#" ? 1 : note === "C" ? 2 : 3;
    await invoke("set_drop_tuning", { enabled, note: noteVal });
    await saveSettings({
      drop_tuning_enabled: enabled,
      drop_tuning_note: note,
    });
  });

  onMounted(async () => {
    try {
      loading.value = true;
      devices.value = await invoke<string[]>("get_audio_devices");

      // settings.yamlから設定を取得
      let settings: Settings = {};
      try {
        settings = await invoke<Settings>("get_settings");
      } catch {}

      if (typeof settings.threshold === "number" && !isNaN(settings.threshold)) {
        threshold.value = settings.threshold;
        await invoke("set_threshold", { ratio: threshold.value });
      } else {
        threshold.value = 2.0;
        await invoke("set_threshold", { ratio: threshold.value });
      }
      if (settings.channel_mode !== undefined && [0, 1, 2].includes(settings.channel_mode)) {
        channelMode.value = settings.channel_mode as ChannelMode;
        await invoke("set_channel_mode", { mode: channelMode.value });
      }

      // 新しい設定の復元
      if (settings.pitch_mode) pitchMode.value = settings.pitch_mode;
      if (settings.custom_pitch) customPitch.value = settings.custom_pitch;
      if (settings.tuning_shift) tuningShift.value = settings.tuning_shift;
      if (typeof settings.drop_tuning_enabled === "boolean")
        dropEnabled.value = settings.drop_tuning_enabled;
      if (settings.drop_tuning_note) dropNote.value = settings.drop_tuning_note;

      // 初期値をバックエンドに送信
      const modeVal =
        pitchMode.value === "standard" ? 0 : pitchMode.value === "custom" ? 1 : 2;
      await invoke("set_pitch_mode", { mode: modeVal });
      await invoke("set_custom_pitch", { pitch: Number(customPitch.value) });
      await invoke("set_tuning_shift", { semitones: Number(tuningShift.value) });
      const noteVal =
        dropNote.value === "D"
          ? 0
          : dropNote.value === "C#"
          ? 1
          : dropNote.value === "C"
          ? 2
          : 3;
      await invoke("set_drop_tuning", { enabled: !!dropEnabled.value, note: noteVal });

      if (settings.device_name && devices.value.includes(settings.device_name)) {
        selectedDevice.value = settings.device_name;
      } else if (devices.value.length > 0) {
        selectedDevice.value = devices.value[0];
      }
      if (selectedDevice.value) {
        await startListening(selectedDevice.value);
      }

      listen("frequency", (event) => {
        if (typeof event.payload === "number") {
          frequency.value = event.payload;
        }
      });
      listen("raw_frequency", (event) => {
        if (typeof event.payload === "number") {
          rawFrequency.value = event.payload;
        }
      });
      listen("input_level", (event) => {
        if (typeof event.payload === "number") {
          inputLevel.value = event.payload;
        }
      });
      listen("reset", () => {
        // 状態を初期化
        frequency.value = null;
        rawFrequency.value = null;
      });
    } catch (e: unknown) {
      error.value = e?.toString() ?? "Error";
    } finally {
      loading.value = false;
    }
  });

  watch(selectedDevice, (newDevice, oldDevice) => {
    if (newDevice && newDevice !== oldDevice) {
      startListening(newDevice);
    }
  });

  return {
    devices,
    selectedDevice,
    loading,
    error,
    listenStatus,
    frequency,
    rawFrequency,
    threshold,
    inputLevel,
    channelMode,
    pitchMode,
    customPitch,
    tuningShift,
    dropEnabled,
    dropNote,
    updateThreshold,
    updateChannelMode,
    saveSettings,
  };
}
