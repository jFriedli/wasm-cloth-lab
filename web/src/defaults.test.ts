import { describe, expect, it } from "vitest";
import { AttachmentMode, FRESH_DEFAULTS, QUALITY_MESH, QualityPreset, resolveInitialSettings } from "./defaults";

describe("initial settings", () => {
  it("uses Top edge and Ultra for a clean state", () => {
    expect(resolveInitialSettings()).toEqual({ attachment: AttachmentMode.TopEdge, quality: QualityPreset.Ultra });
    expect(FRESH_DEFAULTS.attachment).toBe(2);
    expect(QUALITY_MESH[FRESH_DEFAULTS.quality]).toEqual({ w: 100, h: 64 });
  });

  it("preserves explicitly selected alternatives", () => {
    expect(resolveInitialSettings({ attachment: AttachmentMode.FullEdge, quality: QualityPreset.Low }))
      .toEqual({ attachment: AttachmentMode.FullEdge, quality: QualityPreset.Low });
    expect(resolveInitialSettings({ attachment: AttachmentMode.TwoPoint, quality: QualityPreset.High }))
      .toEqual({ attachment: AttachmentMode.TwoPoint, quality: QualityPreset.High });
  });
});
