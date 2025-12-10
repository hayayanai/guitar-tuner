<script setup lang="ts">
import { computed } from "vue";
import { TUNING_SHIFTS, DROP_TUNINGS } from "../composables/constants";
import type { PitchMode } from "../types";

const pitchMode = defineModel<PitchMode>("pitchMode", { required: true });
const customPitch = defineModel<number>("customPitch", { required: true });
const tuningShift = defineModel<number>("tuningShift", { required: true });
const dropEnabled = defineModel<boolean>("dropEnabled", { required: true });
const dropNote = defineModel<string>("dropNote", { required: true });

const pitchError = computed(() => {
  if (pitchMode.value === "custom") {
    if (customPitch.value < 438 || customPitch.value > 445) {
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
        @change="pitchMode = 'standard'"
      />
      <span>Standard (A4 = 440 Hz)</span>
    </label>

    <!-- Custom Pitch -->
    <label class="radio-label">
      <input
        type="radio"
        value="custom"
        :checked="pitchMode === 'custom'"
        @change="pitchMode = 'custom'"
      />
      <span>Custom pitch:</span>
      <input
        type="number"
        :value="customPitch"
        @input="customPitch = Number(($event.target as HTMLInputElement).value)"
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
        @change="pitchMode = 'shift'"
      />
      <span>Tuning shift:</span>
      <select
        :value="tuningShift"
        @change="tuningShift = Number(($event.target as HTMLSelectElement).value)"
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
        @change="dropEnabled = ($event.target as HTMLInputElement).checked"
      />
      <span>Enable drop tuning:</span>
      <select
        :value="dropNote"
        @change="dropNote = ($event.target as HTMLSelectElement).value"
        :disabled="!dropEnabled"
      >
        <option v-for="opt in DROP_TUNINGS" :key="opt.value" :value="opt.value">
          {{ opt.label }}
        </option>
      </select>
    </label>
    <p class="help-text">
      Combined with Tuning Shift, the 6th string drops relative to the shifted tuning.<br />
      (e.g. Half step down + Drop D = Drop C#)
    </p>
  </fieldset>
</template>

<style scoped>
.pitch-input {
  width: 60px;
  margin: 0 var(--space-sm);
  padding: var(--space-xs) var(--space-sm);
  border-radius: 4px;
  border: 1px solid var(--dads-gray-420);
  background: var(--dads-gray-420);
  color: var(--dads-gray-640);
}

.error-text {
  color: var(--semantic-error-main);
  font-size: var(--font-size-xs);
  margin-left: var(--space-sm);
}

select {
  margin-left: var(--space-sm);
  padding: var(--space-xs) var(--space-sm);
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
  margin-left: var(--space-xs);
}

.help-text {
  font-size: var(--font-size-sm);
  color: var(--dads-gray-420);
  margin: var(--space-sm) 0 0 var(--space-lg);
  line-height: 1.4;
}
</style>
