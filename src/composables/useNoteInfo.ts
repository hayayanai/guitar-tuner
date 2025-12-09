import { computed, type Ref } from "vue";
import type { NoteInfo, TuningStatus } from "../types";
import { NOTE_NAMES } from "./constants";

/**
 * 周波数から音名情報を計算するComposable
 * @param frequency 現在の周波数
 * @param customA4 カスタム基準A4周波数（customモード時のみ440以外）
 */
export function useNoteInfo(frequency: Ref<number | null>, customA4: Ref<number>) {
  const noteInfo = computed<NoteInfo>(() => {
    if (frequency.value === null || frequency.value <= 0) {
      return { name: "-", cent: 0, targetFreq: 0 };
    }
    const f = frequency.value;
    // customピッチのみ考慮（shiftは音名判定に影響しない）
    const a4 = customA4.value;

    // 基準A4からの半音数を計算
    const semitones = 12 * Math.log2(f / a4);
    const nearestSemitone = Math.round(semitones);
    const cent = (semitones - nearestSemitone) * 100;

    // MIDI番号計算（A4=69を基準）
    const midiNote = 69 + nearestSemitone;
    const octave = Math.floor(midiNote / 12) - 1;
    const noteIndex = ((midiNote % 12) + 12) % 12;
    const name = NOTE_NAMES[noteIndex] + octave;

    // 目標周波数（最も近い半音の周波数）
    const targetFreq = a4 * Math.pow(2, nearestSemitone / 12);

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
