<script setup lang="ts">
import { computed } from "vue";
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
} from "./components";
import { useAudioDevice, useNoteInfo, GUITAR_NOTES } from "./composables";
import type { ChannelMode } from "./types";

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
  updateThreshold,
  updateChannelMode,
} = useAudioDevice();

const { noteInfo, tuningStatus, centDisplay } = useNoteInfo(frequency);

function handleChannelChange(mode: ChannelMode) {
  updateChannelMode(mode);
}

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
        <StringReference :notes="GUITAR_NOTES" :active-note-name="noteInfo.name" />
      </div>

      <LevelMeter :level="inputLevel" />

      <details class="settings-panel">
        <summary>Settings</summary>

        <!-- 入力設定グループ -->
        <fieldset class="settings-group">
          <legend>Input</legend>
          <DeviceSelector v-model="selectedDevice" :devices="devices" />
          <ChannelSelector :model-value="channelMode" @update:model-value="handleChannelChange" />
          <!-- ステータス表示（入力設定に関連） -->
          <div v-if="listenStatus" class="status" :class="statusClass">
            <span class="status-indicator"></span>
            {{ statusText }}
          </div>
        </fieldset>

        <!-- 感度設定グループ -->
        <fieldset class="settings-group">
          <legend>Sensitivity</legend>
          <ThresholdSlider :model-value="threshold" @update:model-value="updateThreshold" />
        </fieldset>
      </details>
    </div>
  </div>
</template>

<style scoped>
/* デジタル庁デザインシステム準拠 */
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
  font-size: 16px;
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
  font-size: 14px;
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

/* 設定グループ */
.settings-group {
  border: 1px solid var(--color-divider);
  border-radius: var(--radius-md);
  padding: var(--space-md);
  margin-bottom: var(--space-md);
}

.settings-group legend {
  font-size: 14px;
  font-weight: 700;
  color: var(--color-text-secondary);
  padding: 0 var(--space-sm);
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
