export type V3 = { x: number; y: number; z: number };

export type MobilePermissionState = "unsupported" | "needs-permission" | "listening" | "granted" | "denied";
export type AccelerationSource = "linear" | "gravity-fallback" | "none";
type SensorVector = { x?: number | null; y?: number | null; z?: number | null };

export interface MobileMotionSample {
  acceleration?: SensorVector | null;
  accelerationIncludingGravity?: SensorVector | null;
  interval?: number;
}

export interface MobileMotionState {
  raw: V3;
  includingGravity: V3;
  filtered: V3;
  viewportAcceleration: V3;
  gestureVelocity: V3;
  source: AccelerationSource;
  stationary: boolean;
  stale: boolean;
  frequency: number;
  orientation: number;
}

export interface MobileMotionConfig {
  smoothing: number;
  gravityTimeConstant: number;
  biasTimeConstant: number;
  noiseFloor: number;
  stationaryThreshold: number;
  maxAcceleration: number;
  maxGestureVelocity: number;
  velocityDecay: number;
  stationaryDecay: number;
  staleAfter: number;
  resetAfter: number;
}

const ZERO = (): V3 => ({ x: 0, y: 0, z: 0 });
const finite = (value: number | null | undefined) => Number.isFinite(value) ? Number(value) : 0;
const vector = (value?: SensorVector | null): V3 => ({ x: finite(value?.x), y: finite(value?.y), z: finite(value?.z) });
const add = (a: V3, b: V3): V3 => ({ x: a.x + b.x, y: a.y + b.y, z: a.z + b.z });
const sub = (a: V3, b: V3): V3 => ({ x: a.x - b.x, y: a.y - b.y, z: a.z - b.z });
const scale = (v: V3, amount: number): V3 => ({ x: v.x * amount, y: v.y * amount, z: v.z * amount });
const mix = (a: V3, b: V3, amount: number): V3 => add(a, scale(sub(b, a), amount));
const length = (v: V3) => Math.hypot(v.x, v.y, v.z);
const clamp = (v: V3, maximum: number): V3 => {
  const magnitude = length(v);
  return magnitude > maximum ? scale(v, maximum / magnitude) : v;
};
const radialDeadZone = (v: V3, floor: number): V3 => {
  const magnitude = length(v);
  return magnitude <= floor ? ZERO() : scale(v, (magnitude - floor) / magnitude);
};

/** Rotate natural-orientation device axes into visible-screen axes (x right, y up). */
export function deviceToViewport(value: V3, orientationDegrees: number): V3 {
  const radians = ((orientationDegrees % 360) * Math.PI) / 180;
  const cosine = Math.round(Math.cos(radians));
  const sine = Math.round(Math.sin(radians));
  return {
    x: cosine * value.x + sine * value.y,
    y: -sine * value.x + cosine * value.y,
    z: value.z,
  };
}

export function motionPermissionCapability(
  secure: boolean,
  hasDeviceMotion: boolean,
  hasRequestPermission: boolean,
): MobilePermissionState {
  if (!secure || !hasDeviceMotion) return "unsupported";
  return hasRequestPermission ? "needs-permission" : "listening";
}

export const hasUsefulSensorSample = (state: MobileMotionState, threshold = .3) =>
  state.source !== "none" && (length(state.includingGravity) > 2 || length(state.raw) > threshold);

export class MobileMotionEstimator {
  readonly config: MobileMotionConfig;
  private filtered = ZERO();
  private bias = ZERO();
  private gravity?: V3;
  private velocity = ZERO();
  private lastSampleTime?: number;
  private lastState: MobileMotionState = {
    raw: ZERO(), includingGravity: ZERO(), filtered: ZERO(), viewportAcceleration: ZERO(),
    gestureVelocity: ZERO(), source: "none", stationary: true, stale: true, frequency: 0, orientation: 0,
  };

  constructor(config: Partial<MobileMotionConfig> = {}) {
    this.config = {
      smoothing: .34,
      gravityTimeConstant: .8,
      biasTimeConstant: 4,
      noiseFloor: .14,
      stationaryThreshold: .3,
      maxAcceleration: 16,
      maxGestureVelocity: 3.5,
      velocityDecay: .72,
      stationaryDecay: .16,
      staleAfter: 220,
      resetAfter: 1000,
      ...config,
    };
  }

  reset(orientation = 0): MobileMotionState {
    this.filtered = ZERO(); this.bias = ZERO(); this.gravity = undefined; this.velocity = ZERO();
    this.lastSampleTime = undefined;
    this.lastState = { ...this.lastState, raw: ZERO(), includingGravity: ZERO(), filtered: ZERO(),
      viewportAcceleration: ZERO(), gestureVelocity: ZERO(), source: "none", stationary: true,
      stale: true, frequency: 0, orientation };
    return this.state(Number.POSITIVE_INFINITY);
  }

  sample(sample: MobileMotionSample, time: number, orientation: number): MobileMotionState {
    const previousTime = this.lastSampleTime;
    let dt = previousTime === undefined ? Math.max(.001, finite(sample.interval) / 1000 || 1 / 60) : (time - previousTime) / 1000;
    if (previousTime !== undefined && dt > this.config.resetAfter / 1000) {
      this.reset(orientation); this.lastSampleTime = time;
      return { ...this.lastState, orientation };
    }
    if (dt <= 0) {
      this.reset(orientation);
      dt = Math.max(.001, finite(sample.interval) / 1000 || 1 / 60);
    }
    if (dt > this.config.staleAfter / 1000) {
      const staleSeconds = dt - this.config.staleAfter / 1000;
      this.velocity = scale(this.velocity, Math.exp(-staleSeconds / this.config.stationaryDecay));
      this.filtered = scale(this.filtered, Math.exp(-staleSeconds / this.config.stationaryDecay));
    }
    this.lastSampleTime = time;

    const includingGravity = vector(sample.accelerationIncludingGravity);
    let raw: V3;
    let source: AccelerationSource;
    if (sample.acceleration && [sample.acceleration.x, sample.acceleration.y, sample.acceleration.z].some(Number.isFinite)) {
      raw = vector(sample.acceleration);
      source = "linear";
    } else if (sample.accelerationIncludingGravity) {
      if (!this.gravity) this.gravity = includingGravity;
      const gravityAlpha = 1 - Math.exp(-dt / this.config.gravityTimeConstant);
      this.gravity = mix(this.gravity, includingGravity, gravityAlpha);
      raw = sub(includingGravity, this.gravity);
      source = "gravity-fallback";
    } else {
      raw = ZERO();
      source = "none";
    }

    const unbiased = sub(raw, this.bias);
    const smoothing = 1 - Math.pow(1 - this.config.smoothing, dt * 60);
    this.filtered = mix(this.filtered, unbiased, smoothing);
    const stationary = length(this.filtered) < this.config.stationaryThreshold && length(unbiased) < this.config.stationaryThreshold * 1.5;
    if (stationary && source === "linear") {
      const biasAlpha = 1 - Math.exp(-dt / this.config.biasTimeConstant);
      this.bias = mix(this.bias, raw, biasAlpha);
    }
    const viewportAcceleration = clamp(radialDeadZone(deviceToViewport(this.filtered, orientation), this.config.noiseFloor), this.config.maxAcceleration);
    this.velocity = add(this.velocity, scale(viewportAcceleration, dt));
    const decay = Math.exp(-dt / (stationary ? this.config.stationaryDecay : this.config.velocityDecay));
    this.velocity = clamp(scale(this.velocity, decay), this.config.maxGestureVelocity);

    this.lastState = {
      raw, includingGravity, filtered: this.filtered, viewportAcceleration,
      gestureVelocity: this.velocity, source, stationary, stale: false,
      frequency: dt > 0 ? 1 / dt : 0, orientation,
    };
    return this.state(time);
  }

  state(time: number): MobileMotionState {
    if (this.lastSampleTime === undefined) return { ...this.lastState };
    const age = time - this.lastSampleTime;
    if (age <= this.config.staleAfter) return { ...this.lastState, gestureVelocity: { ...this.velocity } };
    const decay = Math.exp(-(age - this.config.staleAfter) / 1000 / this.config.stationaryDecay);
    return {
      ...this.lastState,
      viewportAcceleration: ZERO(), gestureVelocity: scale(this.velocity, decay),
      stationary: true, stale: true,
    };
  }
}

type PermissionMotionConstructor = typeof DeviceMotionEvent & { requestPermission?: () => Promise<"granted" | "denied"> };
type MotionWindow = Window & { DeviceMotionEvent?: PermissionMotionConstructor; orientation?: number };

export class MobileDeviceMotionSource {
  readonly estimator = new MobileMotionEstimator();
  permission: MobilePermissionState;
  active = false;
  private readonly onMotion = (event: DeviceMotionEvent) => {
    const state = this.estimator.sample(event, performance.now(), this.orientation());
    if (hasUsefulSensorSample(state, this.estimator.config.stationaryThreshold)) this.active = true;
  };
  private readonly onOrientation = () => { this.estimator.reset(this.orientation()); };

  constructor(private readonly target: Window = window) {
    const constructor = (target as MotionWindow).DeviceMotionEvent;
    this.permission = motionPermissionCapability(target.isSecureContext, Boolean(constructor), typeof constructor?.requestPermission === "function");
  }

  startPassive() {
    if (this.permission !== "unsupported" && this.permission !== "needs-permission") this.listen();
  }

  async requestPermission(): Promise<MobilePermissionState> {
    const constructor = (this.target as MotionWindow).DeviceMotionEvent;
    try {
      const result = await constructor?.requestPermission?.();
      this.permission = result === "granted" ? "granted" : "denied";
      if (this.permission === "granted") this.listen();
    } catch {
      this.permission = "denied";
    }
    return this.permission;
  }

  state(now = performance.now()) { return this.estimator.state(now); }
  reset() { this.active = false; this.estimator.reset(this.orientation()); }
  private listen() {
    this.target.removeEventListener("devicemotion", this.onMotion);
    this.target.addEventListener("devicemotion", this.onMotion);
    this.target.screen.orientation?.removeEventListener("change", this.onOrientation);
    this.target.screen.orientation?.addEventListener("change", this.onOrientation);
    this.target.removeEventListener("orientationchange", this.onOrientation);
    this.target.addEventListener("orientationchange", this.onOrientation);
  }
  private orientation() {
    const modern = this.target.screen.orientation?.angle;
    return typeof modern === "number" ? modern : (this.target as MotionWindow).orientation ?? 0;
  }
}
