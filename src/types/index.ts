/** ギター弦の情報 */
export interface GuitarNote {
  name: string;
  freq: number;
}

/** 音名情報 */
export interface NoteInfo {
  name: string;
  cent: number;
  targetFreq: number;
}

/** バックエンドから送られてくる音名イベント */
export interface NoteInfoPayload extends NoteInfo {
  tuningStatus: TuningStatus;
}

/** チューニング状態 */
export type TuningStatus = "perfect" | "good" | "off";

/** チャンネルモード */
export type ChannelMode = 0 | 1 | 2; // 0=左, 1=右, 2=両方の平均

/** 基準ピッチモード */
export type PitchMode = "standard" | "custom" | "shift";

/** ドロップチューニング音名 */
export type DropTuningNote = "D" | "C#" | "C" | "B";
