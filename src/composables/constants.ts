import type { GuitarNote } from '../types';

/** ギター弦の標準チューニング周波数 */
export const GUITAR_NOTES: GuitarNote[] = [
  { name: 'E2', freq: 82.41 },
  { name: 'A2', freq: 110.0 },
  { name: 'D3', freq: 146.83 },
  { name: 'G3', freq: 196.0 },
  { name: 'B3', freq: 246.94 },
  { name: 'E4', freq: 329.63 },
];

/** 音名リスト */
export const NOTE_NAMES = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B'] as const;
