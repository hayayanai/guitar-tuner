<script setup lang="ts">
import { computed, ref, watch, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useI18n } from "vue-i18n";

type ThemeMode = "system" | "light" | "dark";
const themeMode = ref<ThemeMode>("system");
const { locale, t } = useI18n();

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
    // Load locale from settings
    if (settings.locale && (settings.locale === "en" || settings.locale === "ja")) {
      locale.value = settings.locale;
    }
  } catch {}
  applyTheme(themeMode.value);
});

// テーマ変更時に反映＆保存
watch(themeMode, async (val) => {
  applyTheme(val);
  await saveSettings({ theme_mode: val });
});

// Locale change handler
watch(locale, async (val) => {
  await invoke("set_locale", { locale: val });
  await saveSettings({ locale: val });
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
import UpdateNotification from "./components/UpdateNotification.vue";
import { useAudioDevice, getEffectiveA4, getGuitarNotes, type Settings } from "./composables";
import type { ChannelMode, DropTuningNote } from "./types";

// Always on top state
const alwaysOnTop = ref(false);
const alwaysOnTopInitialized = ref(false);

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
  noteInfo,
  tuningStatus,
  centDisplay,
  updateThreshold,
  updateChannelMode,
  saveSettings,
} = useAudioDevice();

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

function handleChannelChange(mode: ChannelMode) {
  updateChannelMode(mode);
}

// トレイアイコン表示モード（0=インジケーターのみ, 1=インジケーター+音名, 2=インジケーター+セント値）
const trayIconMode = ref<string>("1");
const trayIconModeInitialized = ref(false);

// 起動時に設定を復元
onMounted(async () => {
  try {
    const settings = await invoke<Settings>("get_settings");
    if (settings.tray_icon_mode !== undefined && [0, 1, 2].includes(settings.tray_icon_mode)) {
      trayIconMode.value = String(settings.tray_icon_mode);
      await invoke("set_tray_icon_mode", { mode: settings.tray_icon_mode });
    }

    // Always on top setting restoration
    if (typeof settings.always_on_top === "boolean") {
      alwaysOnTop.value = settings.always_on_top;
      await invoke("set_always_on_top", { enabled: settings.always_on_top });
    }
  } catch (e) {
    console.error("Failed to load settings:", e);
  } finally {
    trayIconModeInitialized.value = true;
    alwaysOnTopInitialized.value = true;
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

// Notify backend and save settings when always on top changes
watch(alwaysOnTop, async (enabled) => {
  // Ignore changes before initialization (prevent triggering during settings restoration)
  if (!alwaysOnTopInitialized.value) return;

  try {
    await invoke("set_always_on_top", { enabled });
    await saveSettings({ always_on_top: enabled });
  } catch (e) {
    console.error("Failed to set always on top:", e);
  }
});

// ステータス表示の改善
const statusText = computed(() => {
  if (!listenStatus.value) return "";
  if (listenStatus.value.startsWith("Listening:")) return t("status.listening");
  if (listenStatus.value.startsWith("Starting")) return t("status.starting");
  if (listenStatus.value.startsWith("Failed")) return t("status.failed");
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
  <div class="app">
    <UpdateNotification />
    <div class="content">
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
        <summary>{{ t("settings.title") }}</summary>

        <!-- 入力設定グループ -->
        <fieldset class="settings-group">
          <legend>{{ t("settings.input") }}</legend>
          <DeviceSelector v-model="selectedDevice" :devices="devices" />
          <ChannelSelector :model-value="channelMode" @update:model-value="handleChannelChange" />
          <div v-if="listenStatus" class="status" :class="statusClass">
            <span class="status-indicator" />
            {{ statusText }}
          </div>
        </fieldset>

        <!-- 感度設定グループ -->
        <fieldset class="settings-group">
          <legend>{{ t("settings.sensitivity") }}</legend>
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
          <legend>{{ t("settings.trayIcon") }}</legend>
          <div class="tray-mode-selector">
            <label class="radio-label">
              <input v-model="trayIconMode" type="radio" name="trayMode" value="0" />
              <span>{{ t("trayIcon.indicatorOnly") }}</span>
            </label>
            <label class="radio-label">
              <input v-model="trayIconMode" type="radio" name="trayMode" value="1" />
              <span>{{ t("trayIcon.indicatorNote") }}</span>
            </label>
            <label class="radio-label">
              <input v-model="trayIconMode" type="radio" name="trayMode" value="2" />
              <span>{{ t("trayIcon.indicatorCents") }}</span>
            </label>
          </div>
        </fieldset>

        <!-- テーマ設定グループ -->
        <fieldset class="settings-group">
          <legend>{{ t("settings.theme") }}</legend>
          <div class="tray-mode-selector">
            <label class="radio-label">
              <input v-model="themeMode" type="radio" name="themeMode" value="system" />
              <span>{{ t("theme.system") }}</span>
            </label>
            <label class="radio-label">
              <input v-model="themeMode" type="radio" name="themeMode" value="light" />
              <span>{{ t("theme.light") }}</span>
            </label>
            <label class="radio-label">
              <input v-model="themeMode" type="radio" name="themeMode" value="dark" />
              <span>{{ t("theme.dark") }}</span>
            </label>
          </div>
        </fieldset>

        <!-- Language settings group -->
        <fieldset class="settings-group">
          <legend>{{ t("settings.language") }}</legend>
          <div class="tray-mode-selector">
            <label class="radio-label">
              <input v-model="locale" type="radio" name="locale" value="en" />
              <span>{{ t("language.en") }}</span>
            </label>
            <label class="radio-label">
              <input v-model="locale" type="radio" name="locale" value="ja" />
              <span>{{ t("language.ja") }}</span>
            </label>
          </div>
        </fieldset>

        <!-- Always on top settings group -->
        <fieldset class="settings-group">
          <legend>{{ t("settings.window") }}</legend>
          <label class="checkbox-label">
            <input v-model="alwaysOnTop" type="checkbox" />
            <span>{{ t("window.alwaysOnTop") }}</span>
          </label>
        </fieldset>

        <div class="version-label">Version 0.2.4</div>
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

.error {
  text-align: center;
  padding: var(--space-md);
  color: var(--color-error);
  background-color: var(--color-error-light);
  border-radius: var(--radius-md);
}

.settings-panel {
  margin: var(--space-xl) auto 0 auto;
  background-color: var(--color-background-secondary);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  padding: var(--space-md);
  width: 100%;
  box-sizing: border-box;

  summary {
    font-size: var(--font-size-base);
    font-weight: 700;
    color: var(--color-primary);
    cursor: pointer;
    line-height: 1.5;
  }

  &[open] summary {
    margin-bottom: var(--space-md);
    padding-bottom: var(--space-sm);
    border-bottom: 1px solid var(--color-divider);
  }
}

.status {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--space-sm);
  font-size: var(--font-size-sm);
  margin-top: var(--space-sm);
  font-weight: 600;

  .status-indicator {
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background-color: currentColor;
  }

  &.status-success {
    color: var(--color-success);
  }

  &.status-pending {
    color: var(--color-warning);
  }

  &.status-error {
    color: var(--color-error);
  }
}

/* トレイアイコンモード選択 */
.tray-mode-selector {
  display: flex;
  flex-direction: column;
  gap: var(--space-sm);
}

.tuner {
  width: 100%;
  box-sizing: border-box;
}

.content {
  animation: fadeIn 0.3s ease-out;
  width: 400px;
  max-width: calc(100vw - 32px);
  margin: 0 auto;
  box-sizing: border-box;
}

.app {
  position: relative;
  display: flex;
  flex-direction: column;
  align-items: center;
  width: 100%;
  padding: var(--space-md);
  box-sizing: border-box;
  min-height: 100vh;
}

.version-label {
  text-align: right;
  margin-top: var(--space-xl);
  color: var(--color-text-light);
  font-size: var(--font-size-sm);
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
