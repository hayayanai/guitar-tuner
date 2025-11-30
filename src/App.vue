<script setup lang="ts">
import {
  DeviceSelector,
  ChannelSelector,
  LevelMeter,
  ThresholdSlider,
  NoteDisplay,
  TunerMeter,
  CentDisplay,
  RawFrequencyDisplay,
  StringReference,
} from './components';
import { useAudioDevice, useNoteInfo, GUITAR_NOTES } from './composables';
import type { ChannelMode } from './types';

const {
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
} = useAudioDevice();

const { noteInfo, tuningStatus, centDisplay } = useNoteInfo(frequency);

function handleChannelChange(mode: ChannelMode) {
  updateChannelMode(mode);
}
</script>

<template>
  <div class="container">
    <h1>Guitar Tuner</h1>
    <div v-if="loading" class="loading">Loading...</div>
    <div v-else class="content">
      <div v-if="error" class="error">{{ error }}</div>

      <DeviceSelector v-model="selectedDevice" :devices="devices" />

      <ChannelSelector :model-value="channelMode" @update:model-value="handleChannelChange" />

      <div v-if="listenStatus" class="status">{{ listenStatus }}</div>

      <LevelMeter :level="inputLevel" />

      <ThresholdSlider :model-value="threshold" @update:model-value="updateThreshold" />

      <div class="tuner">
        <NoteDisplay :note-info="noteInfo" :tuning-status="tuningStatus" :frequency="frequency" />

        <TunerMeter :note-info="noteInfo" :tuning-status="tuningStatus" />

        <CentDisplay
          :cent-display="centDisplay"
          :tuning-status="tuningStatus"
          :has-frequency="frequency !== null"
        />

        <RawFrequencyDisplay :raw-frequency="rawFrequency" />

        <StringReference :notes="GUITAR_NOTES" :active-note-name="noteInfo.name" />
      </div>
    </div>
  </div>
</template>

<style scoped>
.container {
  min-height: 100vh;
  background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%);
  color: #fff;
  font-family: 'Segoe UI', 'Meiryo', sans-serif;
  padding: 20px;
  box-sizing: border-box;
}
h1 {
  text-align: center;
  font-size: 1.8em;
  margin-bottom: 20px;
  color: #4fc3f7;
}
.loading,
.error {
  text-align: center;
  padding: 20px;
}
.error {
  color: #ef5350;
}
.status {
  text-align: center;
  font-size: 12px;
  color: #81c784;
  margin-bottom: 10px;
}
.tuner {
  max-width: 400px;
  margin: 0 auto;
}
</style>
