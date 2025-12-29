import type { GuitarNote, PitchMode, DropTuningNote } from "../types";

/** ギター弦の標準チューニング周波数 */
export const GUITAR_NOTES: GuitarNote[] = [
  { name: "E2", freq: 82.41 },
  { name: "A2", freq: 110.0 },
  { name: "D3", freq: 146.83 },
  { name: "G3", freq: 196.0 },
  { name: "B3", freq: 246.94 },
  { name: "E4", freq: 329.63 },
];

/** 音名リスト */
export const NOTE_NAMES = [
  "C",
  "C#",
  "D",
  "D#",
  "E",
  "F",
  "F#",
  "G",
  "G#",
  "A",
  "A#",
  "B",
] as const;

export const TUNING_SHIFTS = [
  { value: -1, labelKey: "tuningShift.halfStepDown" },
  { value: -2, labelKey: "tuningShift.wholeStepDown" },
  { value: -3, labelKey: "tuningShift.oneHalfStepDown" },
  { value: -4, labelKey: "tuningShift.twoStepsDown" },
  { value: -5, labelKey: "tuningShift.twoHalfStepsDown" },
] as const;

export const DROP_TUNINGS = [
  { value: "D#", labelKey: "dropTuning.dropDSharp", freq: 77.78 },
  { value: "D", labelKey: "dropTuning.dropD", freq: 73.42 },
  { value: "C#", labelKey: "dropTuning.dropCSharp", freq: 69.3 },
  { value: "C", labelKey: "dropTuning.dropC", freq: 65.41 },
  { value: "B", labelKey: "dropTuning.dropB", freq: 61.74 },
] as const;

/**
 * 設定に基づいて実効A4周波数を計算
 */
export function getEffectiveA4(
  pitchMode: PitchMode,
  customPitch: number,
  tuningShift: number,
): number {
  switch (pitchMode) {
    case "standard":
      return 440.0;
    case "custom":
      return customPitch;
    case "shift":
      return 440.0 * Math.pow(2, tuningShift / 12);
    default:
      return 440.0;
  }
}

/**
 * 設定に基づいてギター弦の音名と周波数を計算
 */
export function getGuitarNotes(
  pitchMode: PitchMode,
  customPitch: number,
  tuningShift: number,
  dropEnabled: boolean,
  dropNote: DropTuningNote,
): GuitarNote[] {
  // 標準チューニングからの半音数 (E2=40, A2=45, D3=50, G3=55, B3=59, E4=64) from A4=69
  const standardSemitones = [40, 45, 50, 55, 59, 64];

  // shiftモードの場合のみ半音シフトを適用（音名が変わる）
  // customモードは基準ピッチが変わるだけで音名は変わらない
  const shift = pitchMode === "shift" ? tuningShift : 0;

  const notes: GuitarNote[] = standardSemitones.map((semitone) => {
    const adjustedSemitone = semitone + shift;

    // 周波数計算: 常に440Hzを基準にして、shiftを考慮
    const freq = 440.0 * Math.pow(2, (adjustedSemitone - 69) / 12);

    // customモードの場合は周波数を調整
    const finalFreq = pitchMode === "custom" ? (freq * customPitch) / 440.0 : freq;

    // 音名計算（shiftにより変わる）
    const midiNote = 69 + adjustedSemitone - 69; // = adjustedSemitone
    const octave = Math.floor(midiNote / 12) - 1;
    const noteIndex = ((midiNote % 12) + 12) % 12;
    const name = NOTE_NAMES[noteIndex] + octave;

    return { name, freq: finalFreq };
  });

  // 6弦ドロップチューニング適用
  if (dropEnabled) {
    const dropFreq = DROP_TUNINGS.find((d) => d.value === dropNote)?.freq ?? 73.42;
    const dropSemitone = 12 * Math.log2(dropFreq / 440) + 69;
    const adjustedDropSemitone = dropSemitone + shift;

    // 周波数計算
    let freq = 440.0 * Math.pow(2, (adjustedDropSemitone - 69) / 12);
    if (pitchMode === "custom") {
      freq = (freq * customPitch) / 440.0;
    }

    const midiNote = Math.round(adjustedDropSemitone);
    const octave = Math.floor(midiNote / 12) - 1;
    const noteIndex = ((midiNote % 12) + 12) % 12;
    const name = NOTE_NAMES[noteIndex] + octave;

    notes[0] = { name, freq };
  }

  return notes;
}
