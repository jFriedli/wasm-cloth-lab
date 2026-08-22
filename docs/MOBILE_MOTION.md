# Mobile physical motion

Mobile motion is a progressive enhancement over the same canonical inputs used by desktop window motion. It does not attempt inertial navigation or absolute phone position.

## Browser APIs and permission

The source listens for the secure-context [`devicemotion`](https://www.w3.org/TR/orientation-event/) event. It prefers `DeviceMotionEvent.acceleration`, uses `accelerationIncludingGravity` only as a fallback, records `interval`, and exposes the raw values in debug mode. `rotationRate` and `DeviceOrientationEvent` are not used: phone twists do not have an unambiguous mapping to translational container motion.

When `DeviceMotionEvent.requestPermission()` exists, the UI shows **Enable phone motion** and invokes it only from that button's click. MDN documents that the method requires transient user activation and resolves to `granted` or `denied`. On implementations without that method, the app listens passively and activates mobile mode only after a non-null motion sample arrives. It therefore uses no user-agent string and does not show a sensor prompt on an ordinary desktop merely because the interface exists.

The feature is disabled outside a secure context. Production is HTTPS. For development, use the deployed site or an HTTPS development origin on the phone; `localhost`'s special trust treatment does not extend to an arbitrary LAN HTTP address.

## Coordinate system

W3C device axes are fixed to the device's natural orientation, normally portrait: positive X points toward the natural screen's right, positive Y toward its top, and positive Z out through the screen. Screen rotation does not rotate those axes.

`screen.orientation.angle` rotates them into current viewport axes (`x` visible-right, `y` visible-up):

```text
x_view = cos(angle) x_device + sin(angle) y_device
y_view = -sin(angle) x_device + cos(angle) y_device
z_view = z_device
```

The estimator resets on Screen Orientation API `change`, preventing the basis change from retaining old gesture velocity. Tests cover 0°, 90°, 180°, and 270°.

## Linear acceleration and gravity fallback

`acceleration` is used when any component is finite. It is the browser's gravity-removed linear acceleration and is the least ambiguous source.

If it is null but `accelerationIncludingGravity` exists, an exponential low-pass gravity estimate (0.8 s time constant) is subtracted. The first fallback sample initializes gravity and creates no impulse. This suppresses a held tilt, but rapid rotation can temporarily look like translation and sustained low-frequency translation can leak into the gravity estimate. Debug mode labels the source rather than pretending both paths have equal quality.

## Gesture velocity—not absolute velocity

Acceleration bias makes unbounded integration unusable. The estimator instead produces a deliberately short-lived gesture velocity:

1. Reject non-finite values and reset after a pause longer than 1 s.
2. Remove gravity when the fallback source is used.
3. Subtract a slowly learned stationary bias.
4. Apply a sample-rate-normalized exponential low-pass (`0.34` nominal coefficient at 60 Hz).
5. Apply a radial 0.14 m/s² dead zone and clamp acceleration to 16 m/s².
6. Integrate once into gesture velocity.
7. Leak velocity toward zero with a 0.72 s moving or 0.16 s stationary time constant.
8. Clamp gesture velocity to 3.5 m/s.
9. After 220 ms without samples, output zero acceleration and rapidly decay remaining velocity.

Stationary detection requires filtered acceleration below 0.30 m/s² and unbiased input below 0.45 m/s². While stationary on the direct-linear source, bias adapts with a 4 s time constant. These constants are interaction tuning, not inertial-navigation calibration.

## Physics mapping

```text
inertial acceleration = -viewport acceleration × 1.6 × Motion inertia slider
relative wind = -gesture velocity × 6.0 × Motion wind slider
```

Y airflow retains the desktop model's 0.35 visual scale. Z wind and inertia use a 0.65 scale and can billow the 3D mesh when the phone moves toward/away from the viewer. Both vectors are bounded before reaching WASM. The Rust solver has no mobile-specific code.

Sensor events update asynchronously; `requestAnimationFrame` reads the latest state and the existing 120 Hz accumulator feeds canonical vectors into WASM. Visibility changes, long pauses, and orientation changes reset history. Touch constraints remain independent and `touch-action: none` stays scoped to the canvas.

## Limitations

- Gesture velocity represents recent motion, not measured phone velocity.
- Slow constant-speed translation is not observable from acceleration alone.
- Sensor bias, browser filtering, quantization, event frequency, and axis quality vary.
- The fallback cannot perfectly distinguish rapid tilt from translation.
- No physical phone was available during implementation; gains require real-device evaluation.
- Embedded WebViews/iframes can have additional restrictions. Production is served directly, not in an iframe.

Sources: W3C [Device Orientation and Motion](https://www.w3.org/TR/orientation-event/), MDN [`devicemotion`](https://developer.mozilla.org/en-US/docs/Web/API/Window/devicemotion_event) and [`requestPermission()`](https://developer.mozilla.org/en-US/docs/Web/API/DeviceMotionEvent/requestPermission_static), WebKit [Safari 26.4 notes](https://webkit.org/blog/17862/webkit-features-for-safari-26-4/), and the current [compatibility table](https://caniuse.com/deviceorientation).
