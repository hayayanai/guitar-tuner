import { ref, onMounted, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { ChannelMode } from "../types";

type Settings = {
  device_name?: string;
  threshold?: number;
  channel_mode?: number;
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
    updateThreshold,
    updateChannelMode,
  };
}
