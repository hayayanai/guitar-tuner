<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import { TUNING_SHIFTS, DROP_TUNINGS } from "../composables/constants";
import type { PitchMode } from "../types";

const { t } = useI18n();

const pitchMode = defineModel<PitchMode>("pitchMode", { required: true });
const customPitch = defineModel<number>("customPitch", { required: true });
const tuningShift = defineModel<number>("tuningShift", { required: true });
const dropEnabled = defineModel<boolean>("dropEnabled", { required: true });
const dropNote = defineModel<string>("dropNote", { required: true });

const pitchError = computed(() => {
  if (pitchMode.value === "custom") {
    if (customPitch.value < 438 || customPitch.value > 445) {
      return t("pitch.errorRange");
    }
  }
  return "";
});
</script>

<template>
  <fieldset class="settings-group">
    <legend>{{ t("settings.referencePitch") }}</legend>

    <!-- Standard -->
    <label class="radio-label">
      <input
        type="radio"
        value="standard"
        :checked="pitchMode === 'standard'"
        @change="pitchMode = 'standard'"
      />
      <span>{{ t("pitch.standard") }}</span>
    </label>

    <!-- Custom Pitch -->
    <label class="radio-label">
      <input
        type="radio"
        value="custom"
        :checked="pitchMode === 'custom'"
        @change="pitchMode = 'custom'"
      />
      <span>{{ t("pitch.custom") }}</span>
      <input
        type="number"
        :value="customPitch"
        :disabled="pitchMode !== 'custom'"
        min="438"
        max="445"
        step="1"
        class="pitch-input"
        @input="customPitch = Number(($event.target as HTMLInputElement).value)"
      />
      <span>{{ t("pitch.hz") }}</span>
      <span class="range-hint">{{ t("pitch.customHint") }}</span>
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
      <span>{{ t("pitch.shift") }}</span>
      <select
        :value="tuningShift"
        :disabled="pitchMode !== 'shift'"
        @change="tuningShift = Number(($event.target as HTMLSelectElement).value)"
      >
        <option v-for="opt in TUNING_SHIFTS" :key="opt.value" :value="opt.value">
          {{ opt.label }}
        </option>
      </select>
    </label>
  </fieldset>

  <fieldset class="settings-group">
    <legend>{{ t("settings.dropTuning") }}</legend>
    <label class="checkbox-label">
      <input
        type="checkbox"
        :checked="dropEnabled"
        @change="dropEnabled = ($event.target as HTMLInputElement).checked"
      />
      <span>{{ t("dropTuning.enable") }}</span>
      <select
        :value="dropNote"
        :disabled="!dropEnabled"
        @change="dropNote = ($event.target as HTMLSelectElement).value"
      >
        <option v-for="opt in DROP_TUNINGS" :key="opt.value" :value="opt.value">
          {{ opt.label }}
        </option>
      </select>
    </label>
    <p class="help-text">
      {{ t("dropTuning.helpText") }}<br />
      {{ t("dropTuning.helpTextExample") }}
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
  box-sizing: border-box;
  flex-shrink: 0;
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
  box-sizing: border-box;
  max-width: 200px;

  &:disabled {
    opacity: 0.5;
    cursor: not-allowed;
    background: var(--dads-gray-420);
  }
}

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
  max-width: 100%;
  word-wrap: break-word;
  overflow-wrap: break-word;
}
</style>
