import { ref, onMounted, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { ChannelMode } from "../types";

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
    localStorage.setItem("threshold", value.toString());
  }

  async function updateChannelMode(mode: ChannelMode) {
    channelMode.value = mode;
    await invoke("set_channel_mode", { mode });
    localStorage.setItem("channelMode", mode.toString());
  }

  async function startListening(device: string) {
    try {
      listenStatus.value = "Starting...";
      await invoke("start_listening", { deviceName: device });
      listenStatus.value = `Listening: ${device}`;
    } catch (e: unknown) {
      listenStatus.value = "Failed: " + (e?.toString() ?? "");
    }
  }

  onMounted(async () => {
    try {
      loading.value = true;
      devices.value = await invoke<string[]>("get_audio_devices");

      // 保存された閾値を復元
      const savedThreshold = localStorage.getItem("threshold");
      if (savedThreshold) {
        threshold.value = parseFloat(savedThreshold);
        await invoke("set_threshold", { ratio: threshold.value });
      }

      // 保存されたチャンネルモードを復元
      const savedChannelMode = localStorage.getItem("channelMode");
      if (savedChannelMode) {
        const parsed = parseInt(savedChannelMode);
        if ([0, 1, 2].includes(parsed)) {
          channelMode.value = parsed as ChannelMode;
          await invoke("set_channel_mode", { mode: channelMode.value });
        }
      }

      const saved = localStorage.getItem("selectedDevice");
      if (saved && devices.value.includes(saved)) {
        selectedDevice.value = saved;
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
    } catch (e: unknown) {
      error.value = e?.toString() ?? "Error";
    } finally {
      loading.value = false;
    }
  });

  watch(selectedDevice, (newDevice, oldDevice) => {
    if (newDevice && newDevice !== oldDevice) {
      localStorage.setItem("selectedDevice", newDevice);
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
