<script setup lang="ts">
import { ref, onMounted, watch, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

const devices = ref<string[]>([]);
const selectedDevice = ref('');
const loading = ref(true);
const error = ref('');
const listenStatus = ref('');
const frequency = ref<number | null>(null);
const rawFrequency = ref<number | null>(null);
const threshold = ref(2.0);
const inputLevel = ref(0);
const channelMode = ref(1); // 0=左, 1=右, 2=平均

const guitarNotes = [
  { name: 'E2', freq: 82.41 },
  { name: 'A2', freq: 110.00 },
  { name: 'D3', freq: 146.83 },
  { name: 'G3', freq: 196.00 },
  { name: 'B3', freq: 246.94 },
  { name: 'E4', freq: 329.63 },
];

const noteNames = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B'];

const noteInfo = computed(() => {
  if (frequency.value === null || frequency.value <= 0) {
    return { name: '-', cent: 0, targetFreq: 0 };
  }
  const f = frequency.value;
  const semitones = 12 * Math.log2(f / 440);
  const nearestSemitone = Math.round(semitones);
  const cent = (semitones - nearestSemitone) * 100;
  const midiNote = 69 + nearestSemitone; // A4 = 69
  const octave = Math.floor(midiNote / 12) - 1;
  const noteIndex = ((midiNote % 12) + 12) % 12;
  const name = noteNames[noteIndex] + octave;
  const targetFreq = 440 * Math.pow(2, nearestSemitone / 12);
  return { name, cent, targetFreq };
});

const tuningStatus = computed(() => {
  const cent = noteInfo.value.cent;
  if (Math.abs(cent) <= 3) return 'perfect';
  if (Math.abs(cent) <= 10) return 'good';
  return 'off';
});

const needleRotation = computed(() => {
  const cent = Math.max(-50, Math.min(50, noteInfo.value.cent));
  return cent * 0.9;
});

// レベルメーター用（-80dB〜0dBを0〜100%に変換）
const levelPercent = computed(() => {
  // inputLevel は 0〜1 の値（-80dB〜0dBに対応）
  return inputLevel.value * 100;
});

// cent表示用（整数、符号付き）
const centDisplay = computed(() => {
  const cent = Math.round(noteInfo.value.cent);
  if (cent > 0) return `+${cent}`;
  return cent.toString();
});

async function updateThreshold(value: number) {
  threshold.value = value;
  await invoke('set_threshold', { ratio: value });
  localStorage.setItem('threshold', value.toString());
}

async function updateChannelMode(mode: number) {
  channelMode.value = mode;
  await invoke('set_channel_mode', { mode });
  localStorage.setItem('channelMode', mode.toString());
}

onMounted(async () => {
  try {
    loading.value = true;
    devices.value = await invoke<string[]>('get_audio_devices');

    // 保存された閾値を復元
    const savedThreshold = localStorage.getItem('threshold');
    if (savedThreshold) {
      threshold.value = parseFloat(savedThreshold);
      await invoke('set_threshold', { ratio: threshold.value });
    }

    // 保存されたチャンネルモードを復元
    const savedChannelMode = localStorage.getItem('channelMode');
    if (savedChannelMode) {
      channelMode.value = parseInt(savedChannelMode);
      await invoke('set_channel_mode', { mode: channelMode.value });
    }

    const saved = localStorage.getItem('selectedDevice');
    if (saved && devices.value.includes(saved)) {
      selectedDevice.value = saved;
    } else if (devices.value.length > 0) {
      selectedDevice.value = devices.value[0];
    }
    if (selectedDevice.value) {
      await startListening(selectedDevice.value);
    }
    listen('frequency', (event) => {
      if (typeof event.payload === 'number') {
        frequency.value = event.payload;
      }
    });
    listen('raw_frequency', (event) => {
      if (typeof event.payload === 'number') {
        rawFrequency.value = event.payload;
      }
    });
    listen('input_level', (event) => {
      if (typeof event.payload === 'number') {
        inputLevel.value = event.payload;
      }
    });
  } catch (e: unknown) {
    error.value = e?.toString() ?? 'Error';
  } finally {
    loading.value = false;
  }
});

async function startListening(device: string) {
  try {
    listenStatus.value = 'Starting...';
    await invoke('start_listening', { deviceName: device });
    listenStatus.value = `Listening: ${device}`;
  } catch (e: unknown) {
    listenStatus.value = 'Failed: ' + (e?.toString() ?? '');
  }
}

watch(selectedDevice, (newDevice, oldDevice) => {
  if (newDevice && newDevice !== oldDevice) {
    localStorage.setItem('selectedDevice', newDevice);
    startListening(newDevice);
  }
});
</script>

<template>
  <div class="container">
    <h1>Guitar Tuner</h1>
    <div v-if="loading" class="loading">Loading...</div>
    <div v-else class="content">
      <div v-if="error" class="error">{{ error }}</div>
      <div class="device-select">
        <label for="device-select">Input:</label>
        <select id="device-select" v-model="selectedDevice">
          <option v-for="d in devices" :key="d" :value="d">{{ d }}</option>
        </select>
      </div>

      <!-- チャンネル選択 -->
      <div class="channel-select">
        <label>Channel:</label>
        <div class="channel-buttons">
          <button :class="{ active: channelMode === 0 }" @click="updateChannelMode(0)">L</button>
          <button :class="{ active: channelMode === 2 }" @click="updateChannelMode(2)">L+R</button>
          <button :class="{ active: channelMode === 1 }" @click="updateChannelMode(1)">R</button>
        </div>
      </div>

      <div v-if="listenStatus" class="status">{{ listenStatus }}</div>

      <!-- 入力レベルメーター -->
      <div class="input-level">
        <label>Input Level</label>
        <div class="level-bar">
          <div class="level-fill" :style="{ width: `${levelPercent}%` }" :class="{ hot: levelPercent > 90 }"></div>
        </div>
        <div class="level-labels">
          <span>-80dB</span>
          <span>-40dB</span>
          <span>0dB</span>
        </div>
      </div>

      <!-- 閾値調整スライダー -->
      <div class="threshold-control">
        <label>Sensitivity: {{ threshold.toFixed(1) }}</label>
        <input
          type="range"
          min="1.2"
          max="5.0"
          step="0.1"
          :value="threshold"
          @input="updateThreshold(parseFloat(($event.target as HTMLInputElement).value))"
        />
        <div class="threshold-labels">
          <span>High</span>
          <span>Low</span>
        </div>
      </div>

      <div class="tuner">
        <div class="note-display" :class="tuningStatus">
          <span class="note-name">{{ noteInfo.name }}</span>
          <span class="current-freq" v-if="frequency">{{ frequency.toFixed(1) }} Hz</span>
          <span class="current-freq" v-else>--- Hz</span>
          <span class="target-freq" v-if="noteInfo.targetFreq > 0">→ {{ noteInfo.targetFreq.toFixed(2) }} Hz</span>
        </div>

        <!-- 横棒メーター -->
        <div class="tuner-meter">
          <div class="meter-track">
            <div class="meter-labels">
              <span>-50</span>
              <span>0</span>
              <span>+50</span>
            </div>
            <div class="meter-bar">
              <div class="center-zone"></div>
              <div class="meter-marks">
                <div v-for="i in 11" :key="i" class="tick" :class="{ center: i === 6 }"></div>
              </div>
              <div class="indicator" :style="{ left: `${50 + noteInfo.cent}%` }" :class="tuningStatus"></div>
            </div>
          </div>
        </div>

        <div class="cent-display" :class="tuningStatus">
          <span v-if="frequency">{{ centDisplay }} cent</span>
          <span v-else>---</span>
        </div>

        <!-- 生の周波数表示 -->
        <div class="raw-frequency">
          <span class="label">Detected:</span>
          <span class="value">{{ rawFrequency ? rawFrequency.toFixed(1) + ' Hz' : '--- Hz' }}</span>
        </div>

        <div class="string-reference">
          <div v-for="note in guitarNotes" :key="note.name" class="string-note" :class="{ active: noteInfo.name === note.name }">
            {{ note.name }}
          </div>
        </div>
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
.loading, .error {
  text-align: center;
  padding: 20px;
}
.error {
  color: #ef5350;
}
.device-select {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  margin-bottom: 8px;
}
.device-select select {
  padding: 6px 10px;
  font-size: 12px;
  border-radius: 6px;
  border: 1px solid #4fc3f7;
  background: #1a1a2e;
  color: #fff;
  max-width: 200px;
}
.channel-select {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  margin-bottom: 8px;
}
.channel-select label {
  font-size: 12px;
  color: #aaa;
}
.channel-buttons {
  display: flex;
  gap: 4px;
}
.channel-buttons button {
  padding: 4px 12px;
  font-size: 11px;
  border: 1px solid #4fc3f7;
  background: #1a1a2e;
  color: #888;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s;
}
.channel-buttons button.active {
  background: #4fc3f7;
  color: #000;
}
.channel-buttons button:hover:not(.active) {
  background: #2a2a3e;
}
.status {
  text-align: center;
  font-size: 12px;
  color: #81c784;
  margin-bottom: 10px;
}
.input-level {
  max-width: 300px;
  margin: 0 auto 15px;
  padding: 10px 15px;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 8px;
}
.input-level label {
  display: block;
  text-align: center;
  font-size: 12px;
  color: #aaa;
  margin-bottom: 8px;
}
.level-bar {
  width: 100%;
  height: 12px;
  background: #222;
  border-radius: 6px;
  overflow: hidden;
}
.level-fill {
  height: 100%;
  background: linear-gradient(90deg, #4caf50, #8bc34a, #ffeb3b);
  transition: width 0.05s ease-out;
  border-radius: 6px;
}
.level-fill.hot {
  background: linear-gradient(90deg, #4caf50, #ffeb3b, #ff5722);
}
.level-labels {
  display: flex;
  justify-content: space-between;
  font-size: 9px;
  color: #666;
  margin-top: 4px;
}
.threshold-control {
  max-width: 300px;
  margin: 0 auto 20px;
  padding: 10px 15px;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 8px;
}
.threshold-control label {
  display: block;
  text-align: center;
  font-size: 12px;
  color: #aaa;
  margin-bottom: 8px;
}
.threshold-control input[type="range"] {
  width: 100%;
  height: 6px;
  -webkit-appearance: none;
  appearance: none;
  background: #333;
  border-radius: 3px;
  outline: none;
}
.threshold-control input[type="range"]::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 18px;
  height: 18px;
  background: #4fc3f7;
  border-radius: 50%;
  cursor: pointer;
}
.threshold-labels {
  display: flex;
  justify-content: space-between;
  font-size: 10px;
  color: #666;
  margin-top: 4px;
}
.tuner {
  max-width: 400px;
  margin: 0 auto;
}
.note-display {
  text-align: center;
  margin-bottom: 15px;
}
.note-name {
  font-size: 4em;
  font-weight: bold;
  display: block;
  line-height: 1;
}
.current-freq {
  font-size: 1.1em;
  color: #888;
  margin-top: 5px;
  display: inline;
}
.target-freq {
  font-size: 1em;
  color: #4fc3f7;
  margin-left: 8px;
}
.note-display.perfect .note-name {
  color: #4caf50;
  text-shadow: 0 0 20px rgba(76, 175, 80, 0.5);
}
.note-display.good .note-name {
  color: #ffeb3b;
}
.note-display.off .note-name {
  color: #ef5350;
}

/* 横棒メーター */
.tuner-meter {
  margin-bottom: 15px;
}
.meter-track {
  padding: 10px 15px;
  background: rgba(0, 0, 0, 0.3);
  border-radius: 10px;
}
.meter-labels {
  display: flex;
  justify-content: space-between;
  font-size: 11px;
  color: #666;
  margin-bottom: 6px;
  padding: 0 5px;
}
.meter-bar {
  position: relative;
  height: 40px;
  background: #1a1a2e;
  border-radius: 6px;
  overflow: hidden;
}
.center-zone {
  position: absolute;
  left: 50%;
  top: 0;
  bottom: 0;
  width: 20px;
  transform: translateX(-50%);
  background: rgba(76, 175, 80, 0.25);
}
.meter-marks {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  display: flex;
  justify-content: space-between;
  align-items: flex-end;
  padding: 0 10%;
}
.tick {
  width: 2px;
  height: 10px;
  background: #444;
}
.tick.center {
  height: 100%;
  background: #4caf50;
  width: 3px;
}
.indicator {
  position: absolute;
  top: 4px;
  bottom: 4px;
  width: 8px;
  background: #ef5350;
  border-radius: 4px;
  transform: translateX(-50%);
  transition: left 0.08s ease-out;
  box-shadow: 0 0 10px rgba(239, 83, 80, 0.5);
}
.indicator.perfect {
  background: #4caf50;
  box-shadow: 0 0 15px rgba(76, 175, 80, 0.7);
}
.indicator.good {
  background: #ffeb3b;
  box-shadow: 0 0 10px rgba(255, 235, 59, 0.5);
}

.cent-display {
  text-align: center;
  font-size: 2em;
  font-weight: bold;
  margin-bottom: 15px;
}
.cent-display.perfect {
  color: #4caf50;
}
.cent-display.good {
  color: #ffeb3b;
}
.cent-display.off {
  color: #ef5350;
}

.raw-frequency {
  text-align: center;
  margin-bottom: 15px;
  padding: 8px;
  background: rgba(0, 0, 0, 0.2);
  border-radius: 6px;
}
.raw-frequency .label {
  font-size: 11px;
  color: #666;
  margin-right: 8px;
}
.raw-frequency .value {
  font-size: 14px;
  color: #4fc3f7;
  font-family: monospace;
}

.string-reference {
  display: flex;
  justify-content: center;
  gap: 10px;
  padding: 15px;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 10px;
}
.string-note {
  width: 45px;
  height: 45px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #2a2a3e;
  border-radius: 50%;
  font-weight: bold;
  font-size: 14px;
  color: #888;
  transition: all 0.2s;
}
.string-note.active {
  background: #4fc3f7;
  color: #000;
  transform: scale(1.1);
  box-shadow: 0 0 15px rgba(79, 195, 247, 0.5);
}
</style>
