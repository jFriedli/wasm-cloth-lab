# Window motion

This document covers the desktop `screenX`/`screenY` source. Phone sensors use a separate estimator and converge at the canonical wind/inertia interface; see [MOBILE_MOTION.md](MOBILE_MOTION.md). Desktop precision and gains are unchanged by mobile support.

The browser samples `window.screenX` and `window.screenY` on animation frames. Screen coordinates are CSS pixels, so no device-pixel-ratio conversion is applied; DPR and zoom affect canvas backing resolution independently. Negative multi-monitor coordinates are valid.

Two distinct mappings are preserved:

```text
relative airflow = world wind - container velocity
effective acceleration = gravity - container acceleration
```

Thus a window moving right produces leftward relative airflow. A window accelerating right produces a leftward inertial pseudo-force. A sudden stop reverses acceleration while the cloth retains velocity, creating overshoot. Rust and TypeScript tests lock these signs.

Finite differences operate on measured elapsed time. An exponential low-pass filter with a `0.28` nominal 60 Hz coefficient is normalized to elapsed time. Velocity and acceleration are independently clamped. Samples with implausibly large position jumps are rejected; samples delayed over 160 ms decay toward zero; pauses over one second reset history. Visibility changes also reset history. These measures suppress integer-coordinate jitter, tab suspension spikes, and many window-manager discontinuities.

Filtered velocity and acceleration then pass through separate symmetric response curves. The velocity curve ignores magnitudes through 12 CSS px/s and saturates at 2,400 px/s; acceleration ignores magnitudes through 120 CSS px/s² and saturates at 8,500 px/s². Between those points a continuous quadratic lift reaches 1.35× near saturation. This rejects coordinate jitter while making deliberate movement progressively stronger without an extreme-input discontinuity.

Browser wind uses a base gain of `0.016` simulation units per mapped CSS px/s; browser inertia uses `0.005` simulation units per mapped CSS px/s². These replace the original shared-slider conversions of `0.006` and `0.0018`, respectively. Separate Window wind and Window inertia sliders apply a 0–2 multiplier and both default to 1. Ambient wind remains unchanged. Reporting quality varies by OS/browser and cannot be feature-detected beyond observing stable samples. Debug mode shows raw, filtered, and mapped derivatives plus both gains.
