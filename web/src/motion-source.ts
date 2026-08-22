import { mapMotion, MotionEstimator, relativeWind, type MotionState, type V2 } from "./motion";
import type { MobileMotionState, V3 } from "./mobile-motion";

export interface CanonicalMotion {
  source: "desktop-window" | "mobile-device";
  relativeWind: V3;
  inertia: V3;
}

export class DesktopWindowMotionSource {
  readonly estimator = new MotionEstimator();
  last: MotionState = this.estimator.reset();

  sample(position: V2, now: number, windScale: number, inertiaScale: number): CanonicalMotion {
    this.last = this.estimator.sample(position, now);
    const velocity = mapMotion(this.last.velocity, 12, 2400);
    const acceleration = mapMotion(this.last.acceleration, 120, 8500);
    const wind = relativeWind({ x: 0, y: 0 }, { x: velocity.x * .016 * windScale, y: -velocity.y * .016 * windScale });
    return {
      source: "desktop-window",
      relativeWind: { x: wind.x, y: wind.y * .35, z: 0 },
      inertia: { x: -acceleration.x * .005 * inertiaScale, y: acceleration.y * .005 * inertiaScale, z: 0 },
    };
  }

  reset(position: V2, now: number) { this.last = this.estimator.reset(position, now); }
}

export const MOBILE_WIND_GAIN = 6;
export const MOBILE_INERTIA_GAIN = 1.6;

export function mobileCanonicalMotion(state: MobileMotionState, windScale: number, inertiaScale: number): CanonicalMotion {
  return {
    source: "mobile-device",
    relativeWind: {
      x: -state.gestureVelocity.x * MOBILE_WIND_GAIN * windScale,
      y: -state.gestureVelocity.y * MOBILE_WIND_GAIN * windScale * .35,
      z: -state.gestureVelocity.z * MOBILE_WIND_GAIN * windScale * .65,
    },
    inertia: {
      x: -state.viewportAcceleration.x * MOBILE_INERTIA_GAIN * inertiaScale,
      y: -state.viewportAcceleration.y * MOBILE_INERTIA_GAIN * inertiaScale,
      z: -state.viewportAcceleration.z * MOBILE_INERTIA_GAIN * inertiaScale * .65,
    },
  };
}
