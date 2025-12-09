<script setup lang="ts">
import { computed, ref, watch, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
type ThemeMode = "system" | "light" | "dark";
const themeMode = ref<ThemeMode>("system");

function applyTheme(mode: ThemeMode) {
  const html = document.documentElement;
  html.classList.remove("theme-light", "theme-dark");
  if (mode === "light") {
    html.classList.add("theme-light");
  } else if (mode === "dark") {
    html.classList.add("theme-dark");
  }
  // system: 何も付与しない（CSSの@mediaに任せる）
}

// テーマ初期化（設定ファイルから）
onMounted(async () => {
  try {
    const settings = await invoke<Settings>("get_settings");
    if (
      settings.theme_mode === "light" ||
      settings.theme_mode === "dark" ||
      settings.theme_mode === "system"
    ) {
      themeMode.value = settings.theme_mode;
    }
  } catch {}
  applyTheme(themeMode.value);
});

// テーマ変更時に反映＆保存
watch(themeMode, async (val) => {
  applyTheme(val);
  await saveSettings({ theme_mode: val });
});
import {
  DeviceSelector,
  ChannelSelector,
  LevelMeter,
  ThresholdSlider,
  NoteDisplay,
  TunerMeter,
  CentDisplay,
  // RawFrequencyDisplay,
  StringReference,
  PitchSettings,
} from "./components";
import {
  useAudioDevice,
  useNoteInfo,
  getEffectiveA4,
  getGuitarNotes,
  type Settings,
} from "./composables";
import type { ChannelMode, DropTuningNote } from "./types";

const {
  devices,
  selectedDevice,
  loading,
  error,
  listenStatus,
  frequency,
  // rawFrequency,
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
} = useAudioDevice();

// 音名判定用のA4周波数（customモードのみ変化、shiftは影響しない）
const customA4 = computed(() => (pitchMode.value === "custom" ? customPitch.value : 440.0));

// 実効A4周波数の計算（目標周波数表示用）
const effectiveA4 = computed(() =>
  getEffectiveA4(pitchMode.value, customPitch.value, tuningShift.value),
);

// ギター弦の音名・周波数を計算
const guitarNotes = computed(() =>
  getGuitarNotes(
    pitchMode.value,
    customPitch.value,
    tuningShift.value,
    dropEnabled.value,
    dropNote.value as DropTuningNote,
  ),
);

const { noteInfo, tuningStatus, centDisplay } = useNoteInfo(frequency, customA4);

function handleChannelChange(mode: ChannelMode) {
  updateChannelMode(mode);
}

// トレイアイコン表示モード（0=インジケーターのみ, 1=インジケーター+音名）
const trayIconMode = ref<string>("1");
const trayIconModeInitialized = ref(false);

// 起動時に設定を復元
onMounted(async () => {
  try {
    const settings = await invoke<Settings>("get_settings");
    if (settings.tray_icon_mode !== undefined && [0, 1].includes(settings.tray_icon_mode)) {
      trayIconMode.value = String(settings.tray_icon_mode);
      await invoke("set_tray_icon_mode", { mode: settings.tray_icon_mode });
    }
  } catch (e) {
    console.error("Failed to load tray icon mode:", e);
  } finally {
    trayIconModeInitialized.value = true;
  }
});

// モード変更時にバックエンドに通知して保存
watch(trayIconMode, async (newMode) => {
  // 初期化前の変更は無視（設定復元時のトリガーを防ぐ）
  if (!trayIconModeInitialized.value) return;

  try {
    const mode = parseInt(newMode);
    await invoke("set_tray_icon_mode", { mode });
    await saveSettings({ tray_icon_mode: mode });
  } catch (e) {
    console.error("Failed to set tray icon mode:", e);
  }
});

// ステータス表示の改善
const statusText = computed(() => {
  if (!listenStatus.value) return "";
  if (listenStatus.value.startsWith("Listening:")) return "Listening";
  if (listenStatus.value.startsWith("Starting")) return "Starting...";
  if (listenStatus.value.startsWith("Failed")) return "Failed";
  return listenStatus.value;
});

const statusClass = computed(() => {
  if (!listenStatus.value) return "";
  if (listenStatus.value.startsWith("Listening:")) return "status-success";
  if (listenStatus.value.startsWith("Starting")) return "status-pending";
  if (listenStatus.value.startsWith("Failed")) return "status-error";
  return "";
});
</script>

<template>
  <div class="container">
    <div v-if="loading" class="loading">Loading...</div>
    <div v-else class="content">
      <div v-if="error" class="error">
        {{ error }}
      </div>

      <div class="tuner">
        <NoteDisplay :note-info="noteInfo" :tuning-status="tuningStatus" :frequency="frequency" />
        <TunerMeter :note-info="noteInfo" :tuning-status="tuningStatus" />
        <CentDisplay
          :cent-display="centDisplay"
          :tuning-status="tuningStatus"
          :has-frequency="frequency !== null"
        />
        <StringReference :notes="guitarNotes" :active-note-name="noteInfo.name" />
      </div>

      <LevelMeter :level="inputLevel" />

      <details class="settings-panel">
        <summary>Settings</summary>

        <!-- 入力設定グループ -->
        <fieldset class="settings-group">
          <legend>Input</legend>
          <DeviceSelector v-model="selectedDevice" :devices="devices" />
          <ChannelSelector :model-value="channelMode" @update:model-value="handleChannelChange" />
          <div v-if="listenStatus" class="status" :class="statusClass">
            <span class="status-indicator" />
            {{ statusText }}
          </div>
        </fieldset>

        <!-- 感度設定グループ -->
        <fieldset class="settings-group">
          <legend>Sensitivity</legend>
          <ThresholdSlider :model-value="threshold" @update:model-value="updateThreshold" />
        </fieldset>

        <!-- ピッチ設定グループ -->
        <PitchSettings
          v-model:pitch-mode="pitchMode"
          v-model:custom-pitch="customPitch"
          v-model:tuning-shift="tuningShift"
          v-model:drop-enabled="dropEnabled"
          v-model:drop-note="dropNote"
        />

        <!-- トレイアイコン設定グループ -->
        <fieldset class="settings-group">
          <legend>Tray Icon</legend>
          <div class="tray-mode-selector">
            <label class="radio-label">
              <input type="radio" name="trayMode" value="0" v-model="trayIconMode" />
              <span>Indicator only</span>
            </label>
            <label class="radio-label">
              <input type="radio" name="trayMode" value="1" v-model="trayIconMode" />
              <span>Indicator + Note name</span>
            </label>
          </div>
        </fieldset>

        <!-- テーマ設定グループ -->
        <fieldset class="settings-group">
          <legend>Theme</legend>
          <div class="tray-mode-selector">
            <label class="radio-label">
              <input type="radio" name="themeMode" value="system" v-model="themeMode" />
              <span>System</span>
            </label>
            <label class="radio-label">
              <input type="radio" name="themeMode" value="light" v-model="themeMode" />
              <span>Light</span>
            </label>
            <label class="radio-label">
              <input type="radio" name="themeMode" value="dark" v-model="themeMode" />
              <span>Dark</span>
            </label>
          </div>
        </fieldset>
      </details>
    </div>
  </div>
</template>

<style scoped>
/* Slider値の表示 */
.threshold-value {
  text-align: center;
  font-size: var(--font-size-sm);
  color: var(--color-text-secondary);
  margin-top: 4px;
}
.container {
  min-height: 100vh;
  background-color: var(--color-background);
  color: var(--color-text);
  padding: var(--space-lg);
  box-sizing: border-box;
}

.loading,
.error {
  text-align: center;
  padding: var(--space-lg);
}

.error {
  color: var(--color-error);
  background-color: var(--color-error-light);
  border-radius: var(--radius-md);
  padding: var(--space-md);
}

.settings-panel {
  margin: var(--space-xl) auto 0 auto;
  max-width: 400px;
  background-color: var(--color-background-secondary);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  padding: var(--space-md);
}

.settings-panel summary {
  font-size: var(--font-size-base);
  font-weight: 700;
  color: var(--color-primary);
  cursor: pointer;
  line-height: 1.5;
}

.settings-panel[open] summary {
  margin-bottom: var(--space-md);
  padding-bottom: var(--space-sm);
  border-bottom: 1px solid var(--color-divider);
}

.status {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--space-sm);
  font-size: var(--font-size-md);
  padding: var(--space-sm) var(--space-md);
  border-radius: var(--radius-md);
  margin-top: var(--space-md);
}

.status-indicator {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background-color: currentColor;
}

.status-success {
  color: var(--color-success);
  background-color: var(--color-success-light);
}

.status-pending {
  color: var(--color-warning);
  background-color: var(--color-warning-light);
}

.status-error {
  color: var(--color-error);
  background-color: var(--color-error-light);
}

/* トレイアイコンモード選択 */
.tray-mode-selector {
  display: flex;
  flex-direction: column;
  gap: var(--space-sm);
}

.tuner {
  max-width: 400px;
  margin: 0 auto;
}

.content {
  animation: fadeIn 0.3s ease-out;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(8px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
</style>
