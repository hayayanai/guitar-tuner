import type { GuitarNote } from "../types";

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
  { value: -1, label: "Half step down (Eb)" },
  { value: -2, label: "Whole step down (D)" },
  { value: -3, label: "1.5 steps down (Db)" },
  { value: -4, label: "2 steps down (C)" },
  { value: -5, label: "2.5 steps down (B)" },
] as const;

export const DROP_TUNINGS = [
  { value: "D", label: "Drop D", freq: 73.42 },
  { value: "C#", label: "Drop C#", freq: 69.3 },
  { value: "C", label: "Drop C", freq: 65.41 },
  { value: "B", label: "Drop B", freq: 61.74 },
] as const;
