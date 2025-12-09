<script setup lang="ts">
import { computed } from "vue";
import { TUNING_SHIFTS, DROP_TUNINGS } from "../composables/constants";
import type { PitchMode } from "../types";

const props = defineProps<{
  pitchMode: PitchMode;
  customPitch: number;
  tuningShift: number;
  dropEnabled: boolean;
  dropNote: string;
}>();

const emit = defineEmits<{
  (e: "update:pitchMode", value: PitchMode): void;
  (e: "update:customPitch", value: number): void;
  (e: "update:tuningShift", value: number): void;
  (e: "update:dropEnabled", value: boolean): void;
  (e: "update:dropNote", value: string): void;
}>();

const pitchError = computed(() => {
  if (props.pitchMode === "custom") {
    if (props.customPitch < 438 || props.customPitch > 445) {
      return "Must be 438-445 Hz";
    }
  }
  return "";
});
</script>

<template>
  <fieldset class="settings-group">
    <legend>Reference Pitch</legend>

    <!-- Standard -->
    <label class="radio-label">
      <input
        type="radio"
        value="standard"
        :checked="pitchMode === 'standard'"
        @change="emit('update:pitchMode', 'standard')"
      />
      <span>Standard (A4 = 440 Hz)</span>
    </label>

    <!-- Custom Pitch -->
    <label class="radio-label">
      <input
        type="radio"
        value="custom"
        :checked="pitchMode === 'custom'"
        @change="emit('update:pitchMode', 'custom')"
      />
      <span>Custom pitch:</span>
      <input
        type="number"
        :value="customPitch"
        @input="emit('update:customPitch', Number(($event.target as HTMLInputElement).value))"
        :disabled="pitchMode !== 'custom'"
        min="438"
        max="445"
        step="1"
        class="pitch-input"
      />
      <span>Hz</span>
      <span class="range-hint">(438-445)</span>
      <span v-if="pitchError" class="error-text">{{ pitchError }}</span>
    </label>

    <!-- Tuning Shift -->
    <label class="radio-label">
      <input
        type="radio"
        value="shift"
        :checked="pitchMode === 'shift'"
        @change="emit('update:pitchMode', 'shift')"
      />
      <span>Tuning shift:</span>
      <select
        :value="tuningShift"
        @change="emit('update:tuningShift', Number(($event.target as HTMLSelectElement).value))"
        :disabled="pitchMode !== 'shift'"
      >
        <option v-for="opt in TUNING_SHIFTS" :key="opt.value" :value="opt.value">
          {{ opt.label }}
        </option>
      </select>
    </label>
  </fieldset>

  <fieldset class="settings-group">
    <legend>6th String Drop Tuning</legend>
    <label class="checkbox-label">
      <input
        type="checkbox"
        :checked="dropEnabled"
        @change="emit('update:dropEnabled', ($event.target as HTMLInputElement).checked)"
      />
      <span>Enable drop tuning:</span>
      <select
        :value="dropNote"
        @change="emit('update:dropNote', ($event.target as HTMLSelectElement).value)"
        :disabled="!dropEnabled"
      >
        <option v-for="opt in DROP_TUNINGS" :key="opt.value" :value="opt.value">
          {{ opt.label }}
        </option>
      </select>
    </label>
    <p class="help-text">
      Combined with Tuning Shift, the 6th string drops relative to the shifted tuning.<br />
      (e.g. Half step down + Drop D = Drop Db)
    </p>
  </fieldset>
</template>

<style scoped>
.pitch-input {
  width: 60px;
  margin: 0 8px;
  padding: 4px 8px;
  border-radius: 4px;
  border: 1px solid var(--dads-gray-420);
  background: var(--dads-gray-420);
  color: var(--dads-gray-640);
}

.error-text {
  color: var(--semantic-error-main);
  font-size: var(--font-size-xs);
  margin-left: 8px;
}

select {
  margin-left: 8px;
  padding: 4px 8px;
  border-radius: 4px;
  background: var(--dads-gray-420);
  color: var(--dads-gray-640);
  border: 1px solid var(--dads-gray-420);
}

select:disabled,
input:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  background: var(--dads-gray-420);
}

.range-hint {
  font-size: var(--font-size-xs);
  color: var(--dads-gray-420);
  margin-left: 4px;
}

.help-text {
  font-size: var(--font-size-sm);
  color: var(--dads-gray-420);
  margin: 8px 0 0 24px;
  line-height: 1.4;
}
</style>
