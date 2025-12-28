<script setup lang="ts">
import { useI18n } from "vue-i18n";

const { t } = useI18n();
const model = defineModel<number>({ required: true });

function handleInput(event: Event) {
  const target = event.target as HTMLInputElement;
  model.value = parseFloat(target.value);
}
</script>

<template>
  <div class="threshold-control">
    <div class="slider-wrapper">
      <input
        id="sensitivity-slider"
        type="range"
        min="1.2"
        max="5.0"
        step="0.1"
        :value="model"
        @input="handleInput"
      />
    </div>
    <div class="threshold-labels">
      <span>{{ t("settings.sensitivityHigh") }}</span>
      <span class="threshold-value">{{ model.toFixed(1) }}</span>
      <span>{{ t("settings.sensitivityLow") }}</span>
    </div>
  </div>
</template>

<style scoped>
.threshold-control {
  width: 100%;

  .slider-wrapper {
    padding: 6px 12px;
    overflow: hidden;

    input[type="range"] {
      display: block;
      width: -webkit-fill-available;
      height: 8px;
      -webkit-appearance: none;
      appearance: none;
      background: var(--dads-gray-420);
      border-radius: 4px;
      outline: none;
      margin: 0;

      &::-webkit-slider-thumb {
        -webkit-appearance: none;
        appearance: none;
        width: 20px;
        height: 20px;
        background: var(--dads-blue-500);
        border: 2px solid var(--dads-white);
        border-radius: 50%;
        cursor: pointer;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);

        &:hover {
          background: var(--dads-blue-600);
        }
      }

      &:focus {
        outline: none;

        &::-webkit-slider-thumb {
          box-shadow: 0 0 0 3px color-mix(in srgb, var(--dads-blue-500) 30%, transparent 70%);
        }
      }
    }
  }

  .threshold-labels {
    display: flex;
    justify-content: space-between;
    font-size: var(--font-size-sm);
    color: var(--dads-gray-640);
    margin-top: var(--space-xs, 4px);
    padding: 0 10px;

    .threshold-value {
      font-size: var(--font-size-sm);
      color: var(--dads-gray-420);
      margin: 0 12px;
    }
  }
}
</style>
