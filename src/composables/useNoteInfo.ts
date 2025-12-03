import { computed, type Ref } from "vue";
import type { NoteInfo, TuningStatus } from "../types";
import { NOTE_NAMES } from "./constants";

/**
 * 周波数から音名情報を計算するComposable
 */
export function useNoteInfo(frequency: Ref<number | null>) {
  const noteInfo = computed<NoteInfo>(() => {
    if (frequency.value === null || frequency.value <= 0) {
      return { name: "-", cent: 0, targetFreq: 0 };
    }
    const f = frequency.value;
    const semitones = 12 * Math.log2(f / 440);
    const nearestSemitone = Math.round(semitones);
    const cent = (semitones - nearestSemitone) * 100;
    const midiNote = 69 + nearestSemitone; // A4 = 69
    const octave = Math.floor(midiNote / 12) - 1;
    const noteIndex = ((midiNote % 12) + 12) % 12;
    const name = NOTE_NAMES[noteIndex] + octave;
    const targetFreq = 440 * Math.pow(2, nearestSemitone / 12);
    return { name, cent, targetFreq };
  });

  const tuningStatus = computed<TuningStatus>(() => {
    const cent = noteInfo.value.cent;
    if (Math.abs(cent) <= 3) return "perfect";
    if (Math.abs(cent) <= 10) return "good";
    return "off";
  });

  const centDisplay = computed(() => {
    const cent = Math.round(noteInfo.value.cent);
    if (cent > 0) return `+${cent}`;
    return cent.toString();
  });

  return {
    noteInfo,
    tuningStatus,
    centDisplay,
  };
}
