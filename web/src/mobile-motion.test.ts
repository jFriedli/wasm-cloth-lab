import { describe, expect, it } from "vitest";
import { deviceToViewport, hasUsefulSensorSample, MobileMotionEstimator, motionPermissionCapability } from "./mobile-motion";
import { mobileCanonicalMotion } from "./motion-source";

const v = (x: number, y = 0, z = 0) => ({ x, y, z });
const estimator = () => new MobileMotionEstimator({ smoothing: 1, noiseFloor: .1, stationaryThreshold: .2 });

describe("device-to-viewport coordinates", () => {
  it("maps portrait axes", () => expect(deviceToViewport(v(2, 3, 4), 0)).toEqual(v(2, 3, 4)));
  it("maps landscape-left (90 degrees)", () => expect(deviceToViewport(v(2, 3, 4), 90)).toEqual(v(3, -2, 4)));
  it("maps landscape-right (270 degrees)", () => expect(deviceToViewport(v(2, 3, 4), 270)).toEqual(v(-3, 2, 4)));
  it("maps upside-down portrait", () => expect(deviceToViewport(v(2, 3, 4), 180)).toEqual(v(-2, -3, 4)));
});

describe("mobile gesture estimator", () => {
  it("rejects zero and noise-floor input", () => {
    const e = estimator();
    expect(e.sample({ acceleration: v(.05), interval: 16 }, 0, 0).viewportAcceleration).toEqual(v(0));
  });
  it("maps right acceleration to left inertia and wind", () => {
    const e = estimator();
    const state = e.sample({ acceleration: v(3), interval: 20 }, 0, 0);
    const mapped = mobileCanonicalMotion(state, 1, 1);
    expect(state.viewportAcceleration.x).toBeGreaterThan(0);
    expect(mapped.inertia.x).toBeLessThan(0);
    expect(mapped.relativeWind.x).toBeLessThan(0);
  });
  it("maps left acceleration symmetrically", () => {
    const right = estimator().sample({ acceleration: v(3), interval: 20 }, 0, 0);
    const left = estimator().sample({ acceleration: v(-3), interval: 20 }, 0, 0);
    expect(left.viewportAcceleration.x).toBeCloseTo(-right.viewportAcceleration.x);
    expect(left.gestureVelocity.x).toBeCloseTo(-right.gestureVelocity.x);
    expect(mobileCanonicalMotion(left, 1, 1).inertia.x).toBeGreaterThan(0);
  });
  it("leaky integration decays rather than drifting", () => {
    const e = estimator(); let state = e.sample({ acceleration: v(4), interval: 20 }, 0, 0);
    const peak = state.gestureVelocity.x;
    for (let time = 20; time <= 1000; time += 20) state = e.sample({ acceleration: v(0), interval: 20 }, time, 0);
    expect(peak).toBeGreaterThan(0); expect(Math.abs(state.gestureVelocity.x)).toBeLessThan(peak * .05);
  });
  it("bounds acceleration and gesture velocity during a synthetic shake", () => {
    const e = estimator(); let state = e.state(0);
    for (let i = 0; i < 500; i++) state = e.sample({ acceleration: v(i % 2 ? 80 : -80, i % 4 < 2 ? 45 : -45, i % 3 ? 30 : -30), interval: 10 }, i * 10, 0);
    expect(Math.hypot(state.viewportAcceleration.x, state.viewportAcceleration.y, state.viewportAcceleration.z)).toBeLessThanOrEqual(16.0001);
    expect(Math.hypot(state.gestureVelocity.x, state.gestureVelocity.y, state.gestureVelocity.z)).toBeLessThanOrEqual(3.5001);
  });
  it("marks missing samples stale and decays velocity", () => {
    const e = estimator(), moving = e.sample({ acceleration: v(5), interval: 20 }, 0, 0);
    const stale = e.state(1000);
    expect(stale.stale).toBe(true); expect(Math.abs(stale.gestureVelocity.x)).toBeLessThan(Math.abs(moving.gestureVelocity.x));
  });
  it("drops the first sample after a long pause", () => {
    const e = estimator(); e.sample({ acceleration: v(3), interval: 20 }, 0, 0);
    const resumed = e.sample({ acceleration: v(16), interval: 20 }, 2000, 0);
    expect(resumed.viewportAcceleration).toEqual(v(0)); expect(resumed.gestureVelocity).toEqual(v(0));
  });
  it("resets cleanly on orientation change", () => {
    const e = estimator(); e.sample({ acceleration: v(5), interval: 20 }, 0, 0);
    expect(e.reset(90).orientation).toBe(90); expect(e.state(1).gestureVelocity).toEqual(v(0));
  });
  it("falls back to gravity estimation without treating initial gravity as motion", () => {
    const e = estimator();
    const initial = e.sample({ acceleration: null, accelerationIncludingGravity: v(0, 9.81), interval: 20 }, 0, 0);
    expect(initial.source).toBe("gravity-fallback"); expect(initial.viewportAcceleration).toEqual(v(0));
    const movement = e.sample({ acceleration: null, accelerationIncludingGravity: v(3, 9.81), interval: 20 }, 20, 0);
    expect(movement.viewportAcceleration.x).toBeGreaterThan(0);
  });
});

describe("permission feature detection", () => {
  it("requires HTTPS and an exposed constructor", () => {
    expect(motionPermissionCapability(false, true, true)).toBe("unsupported");
    expect(motionPermissionCapability(true, false, false)).toBe("unsupported");
  });
  it("distinguishes explicit permission from passive listening", () => {
    expect(motionPermissionCapability(true, true, true)).toBe("needs-permission");
    expect(motionPermissionCapability(true, true, false)).toBe("listening");
  });
});

it("does not treat desktop zero-valued events as a useful mobile sensor", () => {
  const e = estimator();
  expect(hasUsefulSensorSample(e.sample({ acceleration: v(0), accelerationIncludingGravity: v(0), interval: 16 }, 0, 0))).toBe(false);
  expect(hasUsefulSensorSample(e.sample({ acceleration: v(0), accelerationIncludingGravity: v(0, 9.81), interval: 16 }, 16, 0))).toBe(true);
});
