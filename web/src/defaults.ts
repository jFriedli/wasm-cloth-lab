export enum AttachmentMode { FullEdge = 0, TwoPoint = 1, TopEdge = 2 }
export enum QualityPreset { Low = 0, Medium = 1, High = 2, Ultra = 3 }

export const QUALITY_MESH = [
  { w: 30, h: 20 },
  { w: 50, h: 32 },
  { w: 75, h: 48 },
  { w: 100, h: 64 },
] as const;

export interface InitialSettings { attachment: AttachmentMode; quality: QualityPreset }

export const FRESH_DEFAULTS: Readonly<InitialSettings> = Object.freeze({
  attachment: AttachmentMode.TopEdge,
  quality: QualityPreset.Ultra,
});

export function resolveInitialSettings(overrides: Partial<InitialSettings> = {}): InitialSettings {
  return { ...FRESH_DEFAULTS, ...overrides };
}

export function qualityMesh(preset: QualityPreset) { return QUALITY_MESH[preset]; }
